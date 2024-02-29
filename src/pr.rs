use crate::ae_sys;
use crate::AsPtr;

#[derive(Copy, Clone, Debug, Hash)]
pub struct InDataHandle {
    in_data_ptr: *const ae_sys::PR_InData,
}

impl InDataHandle {
    #[inline]
    pub fn from_raw(in_data_ptr: *const ae_sys::PR_InData) -> InDataHandle {
        InDataHandle { in_data_ptr }
    }

    #[inline]
    pub fn as_ptr(self) -> *const ae_sys::PR_InData {
        self.in_data_ptr
    }

    #[inline]
    pub fn pica_basic_handle(self) -> crate::PicaBasicSuiteHandle {
        crate::PicaBasicSuiteHandle::from_raw(unsafe { (*self.in_data_ptr).pica_basicP })
    }

    #[inline]
    pub fn plugin_id(self) -> i32 {
        unsafe { (*self.in_data_ptr).aegp_plug_id }
    }

    // Fixme: do we own this memory???!
    #[inline]
    pub fn reference_context_ptr(self) -> Box<std::os::raw::c_void> {
        unsafe { Box::<std::os::raw::c_void>::from_raw((*self.in_data_ptr).aegp_refconPV) }
    }
}

define_handle_wrapper!(RenderContextHandle, PR_RenderContextH);
define_handle_wrapper!(InstanceDataHandle, PR_InstanceDataH);
define_handle_wrapper!(InstanceContextHandle, PR_InstanceContextH);
define_handle_wrapper!(GlobalContextHandle, PR_GlobalContextH);
define_handle_wrapper!(GlobalDataHandle, PR_GlobalDataH);
define_handle_wrapper!(RenderDataHandle, PR_RenderDataH);

//EffectWorld
/*
// FIXME: wrap this nicely
#[derive(Copy, Clone, Debug, Hash)]
pub struct RenderContextHandle {
    pub render_context_ptr: ae_sys::PR_RenderContextH,
}

impl RenderContextHandle {
    fn as_ptr(&self): ae_sys::PR_RenderContextH {
        render_context_ptr
    }
}*/

define_enum! {
    ae_sys::PrPixelFormat,
    PixelFormat {
        Bgra4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_BGRA_4444_8u,
        Vuya4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_8u,
        Vuya4444_8u709                              = ae_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_8u_709,
        Argb4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_ARGB_4444_8u,
        Bgrx4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_BGRX_4444_8u,
        Vuyx4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_VUYX_4444_8u,
        Vuyx4444_8u709                              = ae_sys::PrPixelFormat_PrPixelFormat_VUYX_4444_8u_709,
        Xrgb4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_XRGB_4444_8u,
        Bgrp4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_BGRP_4444_8u,
        Vuyp4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_VUYP_4444_8u,
        Vuyp4444_8u709                              = ae_sys::PrPixelFormat_PrPixelFormat_VUYP_4444_8u_709,
        Prgb4444_8u                                 = ae_sys::PrPixelFormat_PrPixelFormat_PRGB_4444_8u,
        Bgra4444_16u                                = ae_sys::PrPixelFormat_PrPixelFormat_BGRA_4444_16u,
        Vuya4444_16u                                = ae_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_16u,
        Argb4444_16u                                = ae_sys::PrPixelFormat_PrPixelFormat_ARGB_4444_16u,
        Bgrx4444_16u                                = ae_sys::PrPixelFormat_PrPixelFormat_BGRX_4444_16u,
        Xrgb4444_16u                                = ae_sys::PrPixelFormat_PrPixelFormat_XRGB_4444_16u,
        Bgrp4444_16u                                = ae_sys::PrPixelFormat_PrPixelFormat_BGRP_4444_16u,
        Prgb4444_16u                                = ae_sys::PrPixelFormat_PrPixelFormat_PRGB_4444_16u,
        Bgra4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_BGRA_4444_32f,
        Vuya4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_32f,
        Vuya4444_32f709                             = ae_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_32f_709,
        Argb4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_ARGB_4444_32f,
        Bgrx4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_BGRX_4444_32f,
        Vuyx4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_VUYX_4444_32f,
        Vuyx4444_32f709                             = ae_sys::PrPixelFormat_PrPixelFormat_VUYX_4444_32f_709,
        Xrgb4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_XRGB_4444_32f,
        Bgrp4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_BGRP_4444_32f,
        Vuyp4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_VUYP_4444_32f,
        Vuyp4444_32f709                             = ae_sys::PrPixelFormat_PrPixelFormat_VUYP_4444_32f_709,
        Prgb4444_32f                                = ae_sys::PrPixelFormat_PrPixelFormat_PRGB_4444_32f,
        Rgb444_10u                                  = ae_sys::PrPixelFormat_PrPixelFormat_RGB_444_10u,
        Yuyv422_8u601                               = ae_sys::PrPixelFormat_PrPixelFormat_YUYV_422_8u_601,
        Yuyv422_8u709                               = ae_sys::PrPixelFormat_PrPixelFormat_YUYV_422_8u_709,
        Uyvy422_8u601                               = ae_sys::PrPixelFormat_PrPixelFormat_UYVY_422_8u_601,
        Uyvy422_8u709                               = ae_sys::PrPixelFormat_PrPixelFormat_UYVY_422_8u_709,
        V210422_10u601                              = ae_sys::PrPixelFormat_PrPixelFormat_V210_422_10u_601,
        V210422_10u709                              = ae_sys::PrPixelFormat_PrPixelFormat_V210_422_10u_709,
        Uyvy422_32f601                              = ae_sys::PrPixelFormat_PrPixelFormat_UYVY_422_32f_601,
        Uyvy422_32f709                              = ae_sys::PrPixelFormat_PrPixelFormat_UYVY_422_32f_709,
        Bgra4444_32fLinear                          = ae_sys::PrPixelFormat_PrPixelFormat_BGRA_4444_32f_Linear,
        Bgrp4444_32fLinear                          = ae_sys::PrPixelFormat_PrPixelFormat_BGRP_4444_32f_Linear,
        Bgrx4444_32fLinear                          = ae_sys::PrPixelFormat_PrPixelFormat_BGRX_4444_32f_Linear,
        Argb4444_32fLinear                          = ae_sys::PrPixelFormat_PrPixelFormat_ARGB_4444_32f_Linear,
        Prgb4444_32fLinear                          = ae_sys::PrPixelFormat_PrPixelFormat_PRGB_4444_32f_Linear,
        Xrgb4444_32fLinear                          = ae_sys::PrPixelFormat_PrPixelFormat_XRGB_4444_32f_Linear,
        Rgb444_12uPq709                             = ae_sys::PrPixelFormat_PrPixelFormat_RGB_444_12u_PQ_709,
        Rgb444_12uPqP3                              = ae_sys::PrPixelFormat_PrPixelFormat_RGB_444_12u_PQ_P3,
        Rgb444_12uPq2020                            = ae_sys::PrPixelFormat_PrPixelFormat_RGB_444_12u_PQ_2020,
        Yuv420Mpeg2FramePicturePlanar8u601          = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_601,
        Yuv420Mpeg2FieldPicturePlanar8u601          = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_601,
        Yuv420Mpeg2FramePicturePlanar8u601FullRange = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_601_FullRange,
        Yuv420Mpeg2FieldPicturePlanar8u601FullRange = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_601_FullRange,
        Yuv420Mpeg2FramePicturePlanar8u709          = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_709,
        Yuv420Mpeg2FieldPicturePlanar8u709          = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_709,
        Yuv420Mpeg2FramePicturePlanar8u709FullRange = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_709_FullRange,
        Yuv420Mpeg2FieldPicturePlanar8u709FullRange = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_709_FullRange,
        Yuv420Mpeg4FramePicturePlanar8u601          = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FRAME_PICTURE_PLANAR_8u_601,
        Yuv420Mpeg4FieldPicturePlanar8u601          = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FIELD_PICTURE_PLANAR_8u_601,
        Yuv420Mpeg4FramePicturePlanar8u601FullRange = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FRAME_PICTURE_PLANAR_8u_601_FullRange,
        Yuv420Mpeg4FieldPicturePlanar8u601FullRange = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FIELD_PICTURE_PLANAR_8u_601_FullRange,
        Yuv420Mpeg4FramePicturePlanar8u709          = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FRAME_PICTURE_PLANAR_8u_709,
        Yuv420Mpeg4FieldPicturePlanar8u709          = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FIELD_PICTURE_PLANAR_8u_709,
        Yuv420Mpeg4FramePicturePlanar8u709FullRange = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FRAME_PICTURE_PLANAR_8u_709_FullRange,
        Yuv420Mpeg4FieldPicturePlanar8u709FullRange = ae_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FIELD_PICTURE_PLANAR_8u_709_FullRange,
        NtscDv25                                    = ae_sys::PrPixelFormat_PrPixelFormat_NTSCDV25,
        PalDv25                                     = ae_sys::PrPixelFormat_PrPixelFormat_PALDV25,
        NtscDv50                                    = ae_sys::PrPixelFormat_PrPixelFormat_NTSCDV50,
        PalDv50                                     = ae_sys::PrPixelFormat_PrPixelFormat_PALDV50,
        NtscDv100_720p                              = ae_sys::PrPixelFormat_PrPixelFormat_NTSCDV100_720p,
        PalDv100_720p                               = ae_sys::PrPixelFormat_PrPixelFormat_PALDV100_720p,
        NtscDv100_1080i                             = ae_sys::PrPixelFormat_PrPixelFormat_NTSCDV100_1080i,
        PalDv100_1080i                              = ae_sys::PrPixelFormat_PrPixelFormat_PALDV100_1080i,
        Raw                                         = ae_sys::PrPixelFormat_PrPixelFormat_Raw,
        Invalid                                     = ae_sys::PrPixelFormat_PrPixelFormat_Invalid,
        Any                                         = ae_sys::PrPixelFormat_PrPixelFormat_Any,
    }
}
