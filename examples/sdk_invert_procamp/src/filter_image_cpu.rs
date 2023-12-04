use after_effects_sys as ae_sys;
use crate::InvertProcAmpParams;

pub unsafe extern "C" fn filter_image_8(refcon: *mut std::ffi::c_void, x: i32, y: i32, in_p: *mut ae_sys::PF_Pixel8, out_p: *mut ae_sys::PF_Pixel8) -> ae_sys::PF_Err {
	// Rescale and call the 16-bit version. [0..255] -> [0..32768]
    let mut tmp_src = ae_sys::PF_Pixel16 {
        red:   (((*in_p).red   as u16 * 257) >> 1) + ((*in_p).red   as u16 & 0x1),
        green: (((*in_p).green as u16 * 257) >> 1) + ((*in_p).green as u16 & 0x1),
        blue:  (((*in_p).blue  as u16 * 257) >> 1) + ((*in_p).blue  as u16 & 0x1),
        alpha: (((*in_p).alpha as u16 * 257) >> 1) + ((*in_p).alpha as u16 & 0x1),
    };

	let mut tmp_dst: ae_sys::PF_Pixel16 = std::mem::zeroed();
	let err = filter_image_16(refcon, x, y, &mut tmp_src, &mut tmp_dst);

	if err == ae_sys::PF_Err_NONE as ae_sys::PF_Err {
		(*out_p).red   = ((tmp_dst.red   >> 7) - ((tmp_dst.red   - 1) >> 14)) as u8;
		(*out_p).green = ((tmp_dst.green >> 7) - ((tmp_dst.green - 1) >> 14)) as u8;
		(*out_p).blue  = ((tmp_dst.blue  >> 7) - ((tmp_dst.blue  - 1) >> 14)) as u8;
		(*out_p).alpha = (*in_p).alpha;
	}
	err
}

pub unsafe extern "C" fn filter_image_16(refcon: *mut std::ffi::c_void, _x: i32, _y: i32, in_p: *mut ae_sys::PF_Pixel16, out_p: *mut ae_sys::PF_Pixel16) -> ae_sys::PF_Err {
	let info = &*(refcon as *const InvertProcAmpParams);

	let tmp = ae_sys::PF_Pixel16 {
        red:   32768 - (*in_p).red as u16,
        green: 32768 - (*in_p).green as u16,
        blue:  32768 - (*in_p).blue as u16,
        alpha: 0
    };

	// RGB -> YUV
	let src_y = ((tmp.red as i32 *  9798 + tmp.green as i32 *  19235 + tmp.blue as i32 *  3735) + 16384) >> 15;
	let src_u = ((tmp.red as i32 * -5529 + tmp.green as i32 * -10855 + tmp.blue as i32 * 16384) + 16384) >> 15;
	let src_v = ((tmp.red as i32 * 16384 + tmp.green as i32 * -13720 + tmp.blue as i32 * -2664) + 16384) >> 15;

	// Render ProcAmp
	let contrast   = (info.contrast           * 32768.0).round() as i32;
	let brightness = (info.brightness         * 32768.0).round() as i32;
	let cos_sat    = (info.hue_cos_saturation * 32768.0).round() as i32;
	let sin_hue    = (info.hue_sin_saturation * 32768.0).round() as i32;

	let dst_y = ((contrast * src_y) + brightness * 32768 + 16384) >> 15;
	let dst_u = ((src_u * cos_sat) + (src_v * -sin_hue) + 16384) >> 15;
	let dst_v = ((src_v * cos_sat) + (src_u *  sin_hue) + 16384) >> 15;

	// YUV -> RGB
	let dst_r = (dst_y * 32768 +                  dst_v *  45940 + 16384) >> 15;
	let dst_g = (dst_y * 32768 + dst_u * -11277 + dst_v * -23401 + 16384) >> 15;
	let dst_b = (dst_y * 32768 + dst_u *  58065                  + 16384) >> 15;

	(*out_p).red   = dst_r.max(0).min(32768) as u16;
	(*out_p).green = dst_g.max(0).min(32768) as u16;
	(*out_p).blue  = dst_b.max(0).min(32768) as u16;
	(*out_p).alpha = (*in_p).alpha;

    ae_sys::PF_Err_NONE as ae_sys::PF_Err
}

pub unsafe extern "C" fn filter_image_32(refcon: *mut std::ffi::c_void, _x: i32, _y: i32, in_p: *mut ae_sys::PF_PixelFloat, out_p: *mut ae_sys::PF_PixelFloat) -> ae_sys::PF_Err {
	let info = &*(refcon as *const InvertProcAmpParams);

	let tmp = ae_sys::PF_PixelFloat {
        red:   1.0 - (*in_p).red,
        green: 1.0 - (*in_p).green,
        blue:  1.0 - (*in_p).blue,
        alpha: 0.0
    };

	// RGB -> YUV
	let src_y = tmp.red *  0.299000 + tmp.green *  0.587000 + tmp.blue *  0.114000;
	let src_u = tmp.red * -0.168736 + tmp.green * -0.331264 + tmp.blue *  0.500000;
	let src_v = tmp.red *  0.500000 + tmp.green * -0.418688 + tmp.blue * -0.081312;

	// Render ProcAmp
	let dst_y = (info.contrast * src_y) + info.brightness;
	let dst_u = (src_u * info.hue_cos_saturation) + (src_v * -info.hue_sin_saturation);
	let dst_v = (src_v * info.hue_cos_saturation) + (src_u *  info.hue_sin_saturation);

	// YUV -> RGB
	(*out_p).red   = dst_y +                    dst_v *  1.402000;
	(*out_p).green = dst_y + dst_u * -0.34413 + dst_v * -0.714136;
	(*out_p).blue  = dst_y + dst_u *  1.772;
	(*out_p).alpha = (*in_p).alpha;

    ae_sys::PF_Err_NONE as ae_sys::PF_Err
}
