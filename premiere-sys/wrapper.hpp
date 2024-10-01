// Required for int*_t/uint*_t.
#include <stdint.h>

#include "AEConfig.h"

#include "PrSDKClipRenderSuite.h"
#include "PrSDKDataStreamTypes.h"
#include "PrSDKDevice.h"
#include "PrSDKEffect.h"
#include "PrSDKEntry.h"
#include "PrSDKErrorSuite.h"
#include "PrSDKGPUDeviceSuite.h"
#include "PrSDKGPUFilter.h"
#include "PrSDKGPUImageProcessingSuite.h"
#include "PrSDKMemoryManagerSuite.h"
#include "PrSDKPixelFormat.h"
#include "PrSDKPixelFormatSuite.h"
#include "PrSDKPlugMemory.h"
#include "PrSDKPlugPPix.h"
#include "PrSDKPlugSuites.h"
#include "PrSDKPlugTimeline.h"
#include "PrSDKPlugUtilities.h"
#include "PrSDKPlugWindow.h"
#include "PrSDKPPix2Suite.h"
#include "PrSDKPPixCacheSuite.h"
#include "PrSDKPPixCreator2Suite.h"
#include "PrSDKPPixCreatorSuite.h"
#include "PrSDKPPixSuite.h"
#include "PrSDKQuality.h"
#include "PrSDKRenderCacheType.h"
#include "PrSDKSceneMetaData.h"
#include "PrSDKSequenceInfoSuite.h"
#include "PrSDKSequenceRenderSuite.h"
#include "PrSDKSmartRenderingSuite.h"
#include "PrSDKStringSuite.h"
#include "PrSDKStructs.h"
#include "PrSDKThreadedWorkSuite.h"
#include "PrSDKTimeSuite.h"
#include "PrSDKTypes.h"
#include "PrSDKVideoSegmentProperties.h"
#include "PrSDKVideoSegmentSuite.h"
#include "PrSDKWindowSuite.h"
#ifdef HAS_AE_SDK
#   include "AE_EffectCB.h"
#   include "AE_CacheOnLoadSuite.h"
#   include "PrSDKAESupport.h"
#   include "PrSDKOpaqueEffectDataSuite.h"
#   include "Smart_Utils.h"
#endif

#include "SPBasic.h"

// GPU format is defined elsewhere in the SDK, so copy it here
#define MAKE_PIXEL_FORMAT_FOURCC(ch0, ch1, ch2, ch3) ((uint32_t)(uint8_t)(ch0) | ((uint32_t)(uint8_t)(ch1) << 8) | ((uint32_t)(uint8_t)(ch2) << 16) | ((uint32_t)(uint8_t)(ch3) << 24 ))
enum PrPixelFormatGpu {
    GPU_BGRA_4444_32f = MAKE_PIXEL_FORMAT_FOURCC('@', 'C', 'D', 'A'), // GPU, BGRA, 32 bits floating point per channel.
    GPU_BGRA_4444_16f = MAKE_PIXEL_FORMAT_FOURCC('@', 'C', 'D', 'a'), // GPU, BGRA, 16 bits floating point per channel.
};
