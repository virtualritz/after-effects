
use crate::*;

define_enum! {
    pr_sys::PrPPixBufferAccess,
    PPixBufferAccess {
        ReadOnly      = pr_sys::PrPPixBufferAccess_PrPPixBufferAccess_ReadOnly,
        WriteOnly     = pr_sys::PrPPixBufferAccess_PrPPixBufferAccess_WriteOnly,
        ReadWrite     = pr_sys::PrPPixBufferAccess_PrPPixBufferAccess_ReadWrite,
        ForceEnumSize = pr_sys::PrPPixBufferAccess_PrPPixBufferAccess_ForceEnumSize,
    }
}
define_enum! {
    pr_sys::PrGPUDeviceFramework,
    GPUDeviceFramework {
        Cuda   = pr_sys::PrGPUDeviceFramework_PrGPUDeviceFramework_CUDA,
        OpenCl = pr_sys::PrGPUDeviceFramework_PrGPUDeviceFramework_OpenCL,
        Metal  = pr_sys::PrGPUDeviceFramework_PrGPUDeviceFramework_Metal,
    }
}

define_enum! {
    pr_sys::PrRenderQuality,
    RenderQuality {
        Max     = pr_sys::PrRenderQuality_kPrRenderQuality_Max,
        High    = pr_sys::PrRenderQuality_kPrRenderQuality_High,
        Medium  = pr_sys::PrRenderQuality_kPrRenderQuality_Medium,
        Low     = pr_sys::PrRenderQuality_kPrRenderQuality_Low,
        Draft   = pr_sys::PrRenderQuality_kPrRenderQuality_Draft,
        Invalid = pr_sys::PrRenderQuality_kPrRenderQuality_Invalid,
    }
}

define_enum! {
    pr_sys::PrPlaybackQuality,
    PlaybackQuality {
        Auto    = pr_sys::PrPlaybackQuality_kPrPlaybackQuality_Auto,
        High    = pr_sys::PrPlaybackQuality_kPrPlaybackQuality_High,
        Draft   = pr_sys::PrPlaybackQuality_kPrPlaybackQuality_Draft,
        Invalid = pr_sys::PrPlaybackQuality_kPrPlaybackQuality_Invalid,
    }
}

define_enum! {
    pr_sys::PrPlaybackFractionalResolution,
    PlaybackFractionalResolution {
        Full      = pr_sys::PrPlaybackFractionalResolution_kPrPlaybackFractionalResolution_Full,
        Half      = pr_sys::PrPlaybackFractionalResolution_kPrPlaybackFractionalResolution_Half,
        Quarter   = pr_sys::PrPlaybackFractionalResolution_kPrPlaybackFractionalResolution_Quarter,
        Eighth    = pr_sys::PrPlaybackFractionalResolution_kPrPlaybackFractionalResolution_Eighth,
        Sixteenth = pr_sys::PrPlaybackFractionalResolution_kPrPlaybackFractionalResolution_Sixteenth,
        Invalid   = pr_sys::PrPlaybackFractionalResolution_kPrPlaybackFractionalResolution_Invalid,
    }
}

define_enum! {
    pr_sys::PrVideoFrameRates,
    VideoFrameRates {
        /// 24000 / 1001
        FrameRate24Drop = pr_sys::PrVideoFrameRates_kVideoFrameRate_24Drop,
        /// 24
        FrameRate24     = pr_sys::PrVideoFrameRates_kVideoFrameRate_24,
        /// 25
        FrameRatePal    = pr_sys::PrVideoFrameRates_kVideoFrameRate_PAL,
        /// 30000 / 1001
        FrameRateNtsc   = pr_sys::PrVideoFrameRates_kVideoFrameRate_NTSC,
        /// 30
        FrameRate30     = pr_sys::PrVideoFrameRates_kVideoFrameRate_30,
        /// 50
        FrameRatePalHd  = pr_sys::PrVideoFrameRates_kVideoFrameRate_PAL_HD,
        /// 60000 / 1001
        FrameRateNtscHd = pr_sys::PrVideoFrameRates_kVideoFrameRate_NTSC_HD,
        /// 60
        FrameRate60     = pr_sys::PrVideoFrameRates_kVideoFrameRate_60,
    }
}

define_enum! {
    pr_sys::pmFieldDisplay,
    FieldDisplay {
        ShowFirstField  = pr_sys::pmFieldDisplay_pmFieldDisplay_ShowFirstField,
        ShowSecondField = pr_sys::pmFieldDisplay_pmFieldDisplay_ShowSecondField,
        ShowBothFields  = pr_sys::pmFieldDisplay_pmFieldDisplay_ShowBothFields,
        ForceSize       = pr_sys::pmFieldDisplay_pmFieldDisplay_ForceSize,
    }
}

define_enum! {
    pr_sys::PrPixelFormat,
    PixelFormat {
        Bgra4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_BGRA_4444_8u,
        Vuya4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_8u,
        Vuya4444_8u709                              = pr_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_8u_709,
        Argb4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_ARGB_4444_8u,
        Bgrx4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_BGRX_4444_8u,
        Vuyx4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_VUYX_4444_8u,
        Vuyx4444_8u709                              = pr_sys::PrPixelFormat_PrPixelFormat_VUYX_4444_8u_709,
        Xrgb4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_XRGB_4444_8u,
        Bgrp4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_BGRP_4444_8u,
        Vuyp4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_VUYP_4444_8u,
        Vuyp4444_8u709                              = pr_sys::PrPixelFormat_PrPixelFormat_VUYP_4444_8u_709,
        Prgb4444_8u                                 = pr_sys::PrPixelFormat_PrPixelFormat_PRGB_4444_8u,
        Bgra4444_16u                                = pr_sys::PrPixelFormat_PrPixelFormat_BGRA_4444_16u,
        Vuya4444_16u                                = pr_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_16u,
        Argb4444_16u                                = pr_sys::PrPixelFormat_PrPixelFormat_ARGB_4444_16u,
        Bgrx4444_16u                                = pr_sys::PrPixelFormat_PrPixelFormat_BGRX_4444_16u,
        Xrgb4444_16u                                = pr_sys::PrPixelFormat_PrPixelFormat_XRGB_4444_16u,
        Bgrp4444_16u                                = pr_sys::PrPixelFormat_PrPixelFormat_BGRP_4444_16u,
        Prgb4444_16u                                = pr_sys::PrPixelFormat_PrPixelFormat_PRGB_4444_16u,
        Bgra4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_BGRA_4444_32f,
        Vuya4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_32f,
        Vuya4444_32f709                             = pr_sys::PrPixelFormat_PrPixelFormat_VUYA_4444_32f_709,
        Argb4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_ARGB_4444_32f,
        Bgrx4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_BGRX_4444_32f,
        Vuyx4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_VUYX_4444_32f,
        Vuyx4444_32f709                             = pr_sys::PrPixelFormat_PrPixelFormat_VUYX_4444_32f_709,
        Xrgb4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_XRGB_4444_32f,
        Bgrp4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_BGRP_4444_32f,
        Vuyp4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_VUYP_4444_32f,
        Vuyp4444_32f709                             = pr_sys::PrPixelFormat_PrPixelFormat_VUYP_4444_32f_709,
        Prgb4444_32f                                = pr_sys::PrPixelFormat_PrPixelFormat_PRGB_4444_32f,
        Rgb444_10u                                  = pr_sys::PrPixelFormat_PrPixelFormat_RGB_444_10u,
        Yuyv422_8u601                               = pr_sys::PrPixelFormat_PrPixelFormat_YUYV_422_8u_601,
        Yuyv422_8u709                               = pr_sys::PrPixelFormat_PrPixelFormat_YUYV_422_8u_709,
        Uyvy422_8u601                               = pr_sys::PrPixelFormat_PrPixelFormat_UYVY_422_8u_601,
        Uyvy422_8u709                               = pr_sys::PrPixelFormat_PrPixelFormat_UYVY_422_8u_709,
        V210422_10u601                              = pr_sys::PrPixelFormat_PrPixelFormat_V210_422_10u_601,
        V210422_10u709                              = pr_sys::PrPixelFormat_PrPixelFormat_V210_422_10u_709,
        Uyvy422_32f601                              = pr_sys::PrPixelFormat_PrPixelFormat_UYVY_422_32f_601,
        Uyvy422_32f709                              = pr_sys::PrPixelFormat_PrPixelFormat_UYVY_422_32f_709,
        Bgra4444_32fLinear                          = pr_sys::PrPixelFormat_PrPixelFormat_BGRA_4444_32f_Linear,
        Bgrp4444_32fLinear                          = pr_sys::PrPixelFormat_PrPixelFormat_BGRP_4444_32f_Linear,
        Bgrx4444_32fLinear                          = pr_sys::PrPixelFormat_PrPixelFormat_BGRX_4444_32f_Linear,
        Argb4444_32fLinear                          = pr_sys::PrPixelFormat_PrPixelFormat_ARGB_4444_32f_Linear,
        Prgb4444_32fLinear                          = pr_sys::PrPixelFormat_PrPixelFormat_PRGB_4444_32f_Linear,
        Xrgb4444_32fLinear                          = pr_sys::PrPixelFormat_PrPixelFormat_XRGB_4444_32f_Linear,
        Rgb444_12uPq709                             = pr_sys::PrPixelFormat_PrPixelFormat_RGB_444_12u_PQ_709,
        Rgb444_12uPqP3                              = pr_sys::PrPixelFormat_PrPixelFormat_RGB_444_12u_PQ_P3,
        Rgb444_12uPq2020                            = pr_sys::PrPixelFormat_PrPixelFormat_RGB_444_12u_PQ_2020,
        Yuv420Mpeg2FramePicturePlanar8u601          = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_601,
        Yuv420Mpeg2FieldPicturePlanar8u601          = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_601,
        Yuv420Mpeg2FramePicturePlanar8u601FullRange = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_601_FullRange,
        Yuv420Mpeg2FieldPicturePlanar8u601FullRange = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_601_FullRange,
        Yuv420Mpeg2FramePicturePlanar8u709          = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_709,
        Yuv420Mpeg2FieldPicturePlanar8u709          = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_709,
        Yuv420Mpeg2FramePicturePlanar8u709FullRange = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FRAME_PICTURE_PLANAR_8u_709_FullRange,
        Yuv420Mpeg2FieldPicturePlanar8u709FullRange = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG2_FIELD_PICTURE_PLANAR_8u_709_FullRange,
        Yuv420Mpeg4FramePicturePlanar8u601          = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FRAME_PICTURE_PLANAR_8u_601,
        Yuv420Mpeg4FieldPicturePlanar8u601          = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FIELD_PICTURE_PLANAR_8u_601,
        Yuv420Mpeg4FramePicturePlanar8u601FullRange = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FRAME_PICTURE_PLANAR_8u_601_FullRange,
        Yuv420Mpeg4FieldPicturePlanar8u601FullRange = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FIELD_PICTURE_PLANAR_8u_601_FullRange,
        Yuv420Mpeg4FramePicturePlanar8u709          = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FRAME_PICTURE_PLANAR_8u_709,
        Yuv420Mpeg4FieldPicturePlanar8u709          = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FIELD_PICTURE_PLANAR_8u_709,
        Yuv420Mpeg4FramePicturePlanar8u709FullRange = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FRAME_PICTURE_PLANAR_8u_709_FullRange,
        Yuv420Mpeg4FieldPicturePlanar8u709FullRange = pr_sys::PrPixelFormat_PrPixelFormat_YUV_420_MPEG4_FIELD_PICTURE_PLANAR_8u_709_FullRange,
        NtscDv25                                    = pr_sys::PrPixelFormat_PrPixelFormat_NTSCDV25,
        PalDv25                                     = pr_sys::PrPixelFormat_PrPixelFormat_PALDV25,
        NtscDv50                                    = pr_sys::PrPixelFormat_PrPixelFormat_NTSCDV50,
        PalDv50                                     = pr_sys::PrPixelFormat_PrPixelFormat_PALDV50,
        NtscDv100_720p                              = pr_sys::PrPixelFormat_PrPixelFormat_NTSCDV100_720p,
        PalDv100_720p                               = pr_sys::PrPixelFormat_PrPixelFormat_PALDV100_720p,
        NtscDv100_1080i                             = pr_sys::PrPixelFormat_PrPixelFormat_NTSCDV100_1080i,
        PalDv100_1080i                              = pr_sys::PrPixelFormat_PrPixelFormat_PALDV100_1080i,
        Raw                                         = pr_sys::PrPixelFormat_PrPixelFormat_Raw,
        Invalid                                     = pr_sys::PrPixelFormat_PrPixelFormat_Invalid,
        Any                                         = pr_sys::PrPixelFormat_PrPixelFormat_Any,
        GpuBgra4444_32f                             = pr_sys::PrPixelFormatGpu_GPU_BGRA_4444_32f,
        GpuBgra4444_16f                             = pr_sys::PrPixelFormatGpu_GPU_BGRA_4444_16f,
    }
}

define_enum! {
    pr_sys::PrKeyframeInterpolationModeFlag,
    KeyframeInterpolationMode {
        Linear                = pr_sys::PrKeyframeInterpolationModeFlag_kPrInterpolationModeFlag_Linear,
        Hold                  = pr_sys::PrKeyframeInterpolationModeFlag_kPrInterpolationModeFlag_Hold,
        Bezier                = pr_sys::PrKeyframeInterpolationModeFlag_kPrInterpolationModeFlag_Bezier,
        Time                  = pr_sys::PrKeyframeInterpolationModeFlag_kPrInterpolationModeFlag_Time,
        TimeTransitionStart   = pr_sys::PrKeyframeInterpolationModeFlag_kPrInterpolationModeFlag_TimeTransitionStart,
        TimeTransitionEnd     = pr_sys::PrKeyframeInterpolationModeFlag_kPrInterpolationModeFlag_TimeTransitionEnd,
        MaxSize               = pr_sys::PrKeyframeInterpolationModeFlag_kPrInterpolationModeFlag_MaxSize,
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Param {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    Point(pr_sys::prFPoint64),
    Guid(pr_sys::prPluginID),
    MemoryPtr(*mut std::ffi::c_char),
}

impl From<pr_sys::PrParam> for Param {
    fn from(value: pr_sys::PrParam) -> Self {
        use pr_sys::*;
        #[allow(non_upper_case_globals)]
        match value.mType {
            PrParamType_kPrParamType_Int8        => Param::Int8     (unsafe { value.__bindgen_anon_1.mInt8 }),
            PrParamType_kPrParamType_Int16       => Param::Int16    (unsafe { value.__bindgen_anon_1.mInt16 }),
            PrParamType_kPrParamType_Int32       => Param::Int32    (unsafe { value.__bindgen_anon_1.mInt32 }),
            PrParamType_kPrParamType_Int64       => Param::Int64    (unsafe { value.__bindgen_anon_1.mInt64 }),
            PrParamType_kPrParamType_Float32     => Param::Float32  (unsafe { value.__bindgen_anon_1.mFloat32 }),
            PrParamType_kPrParamType_Float64     => Param::Float64  (unsafe { value.__bindgen_anon_1.mFloat64 }),
            PrParamType_kPrParamType_Bool        => Param::Bool     (unsafe { value.__bindgen_anon_1.mBool == 1 }),
            PrParamType_kPrParamType_Point       => Param::Point    (unsafe { value.__bindgen_anon_1.mPoint }),
            PrParamType_kPrParamType_Guid        => Param::Guid     (unsafe { value.__bindgen_anon_1.mGuid }),
            PrParamType_kPrParamType_PrMemoryPtr => Param::MemoryPtr(unsafe { value.__bindgen_anon_1.mMemoryPtr }),
            _ => panic!("Invalid PrParamType"),
        }
    }
}
impl Into<pr_sys::PrParam> for Param {
    fn into(self) -> pr_sys::PrParam {
        use pr_sys::*;
        let mut param = PrParam {
            mType: PrParamType_kPrParamType_MaxSize,
            __bindgen_anon_1: PrParam__bindgen_ty_1 { mInt8: 0 },
        };
        match self {
            Param::Int8     (value) => { param.mType = PrParamType_kPrParamType_Int8;        param.__bindgen_anon_1.mInt8 = value; },
            Param::Int16    (value) => { param.mType = PrParamType_kPrParamType_Int16;       param.__bindgen_anon_1.mInt16 = value; },
            Param::Int32    (value) => { param.mType = PrParamType_kPrParamType_Int32;       param.__bindgen_anon_1.mInt32 = value; },
            Param::Int64    (value) => { param.mType = PrParamType_kPrParamType_Int64;       param.__bindgen_anon_1.mInt64 = value; },
            Param::Float32  (value) => { param.mType = PrParamType_kPrParamType_Float32;     param.__bindgen_anon_1.mFloat32 = value; },
            Param::Float64  (value) => { param.mType = PrParamType_kPrParamType_Float64;     param.__bindgen_anon_1.mFloat64 = value; },
            Param::Bool     (value) => { param.mType = PrParamType_kPrParamType_Bool;        param.__bindgen_anon_1.mBool = if value { 1 } else { 0 }; },
            Param::Point    (value) => { param.mType = PrParamType_kPrParamType_Point;       param.__bindgen_anon_1.mPoint = value; },
            Param::Guid     (value) => { param.mType = PrParamType_kPrParamType_Guid;        param.__bindgen_anon_1.mGuid = value; },
            Param::MemoryPtr(value) => { param.mType = PrParamType_kPrParamType_PrMemoryPtr; param.__bindgen_anon_1.mMemoryPtr = value; },
        }
        param
    }
}
