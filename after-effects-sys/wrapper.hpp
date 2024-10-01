// Required for int*_t/uint*_t.
#include <stdint.h>

// Headers subfolder.
#include "AEConfig.h"
#include "A.h"

// Next header makes bindgen generate code that rustc can't handle
// (yet). See https://github.com/rust-lang/rust/issues/59154
// #include "AEFX_SuiteHandlerTemplate.h"

#include "AE_AdvEffectSuites.h"
#include "AE_CacheOnLoadSuite.h"
#include "AE_ChannelSuites.h"
#include "AE_CreatorInfo.h"
#include "AE_Effect.h"
#include "AE_EffectCB.h"
#include "AE_EffectCBSuites.h"
#include "AE_EffectGPUSuites.h"
#include "AE_EffectPixelFormat.h"
#include "AE_EffectSuites.h"
#include "AE_EffectSuitesHelper.h"
#include "AE_EffectUI.h"
#include "AE_EffectVers.h"
#include "AE_GeneralPlug.h"
#include "AE_Hook.h"
#include "AE_IO.h"
#include "AE_IO_FileExt.h"
#include "AE_Macros.h"
#include "AE_PluginData.h"
#include "FIEL_Public.h"
#include "Mach-O_prefix.h"
#include "PF_Masks.h"
//#include "PIFormatT.h"
#include "PR_Public.h"
#include "PT_Public.h"
#include "PrSDKAESupport.h"
#include "PrSDKPixelFormat.h"
#include "SuiteHelper.h"

// Headers/SP subfolder
//#include "SPAccess.h"
//#include "SPAdapts.h"
#include "SPBasic.h"
//#include "SPBckDbg.h"
//#include "SPBlocks.h"
//#include "SPCOM.h"
//#include "SPCaches.h"
//#include "SPConfig.h"
//#include "SPErrorCodes.h"
//#include "SPErrors.h"
//#include "SPFiles.h"
//#include "SPHost.h"
//#include "SPInterf.h"
//#include "SPMData.h"
//#include "SPPiPL.h"
//#include "SPPlugs.h"
//#include "SPProps.h"
//#include "SPRuntme.h"
//#include "SPSTSPrp.h"
//#include "SPStrngs.h"
//#include "SPSuites.h"
//#include "SPTypes.h"

// Util subfolder
//#include "AEFX_ChannelDepthTpl.h"
//#include "DuckSuite.h"
//#include "Param_Utils.h"
#include "entry.h"

// adobesdk subfolder.
#include "adobesdk/DrawbotSuite.h"

// Artisan 2.0 headers.
// Not included with the SDK. Ask Asobe nicely to obtain a copy.
#ifdef ARTISAN_2_API
    #include "AE_Scene3D_Private.h"
    #include "PR_Feature.h"
#endif

// This is a copy from Premiere SDK.
// After effects returns a PrSDKString from a few functions, but the PrSDKStringSuite is not included in the AfterEffects SDK
typedef csSDK_int32 prSuiteError;
enum {
    suiteError_InvalidParms         = 0x80000001, // A parameter to this method is invalid
    suiteError_StringNotFound       = 0x800A0000,
    suiteError_StringBufferTooSmall = 0x800A0001
};
#define kPrSDKStringSuite           "MediaCore StringSuite"
#define kPrSDKStringSuiteVersion    1

#pragma pack(push, 1)
typedef struct {
    prSuiteError (*DisposeString)(const PrSDKString *inSDKString);
    prSuiteError (*AllocateFromUTF8)(const uint8_t *inUTF8String, PrSDKString *outSDKString);
    prSuiteError (*CopyToUTF8String)(const PrSDKString *inSDKString, uint8_t *outUTF8StringBuffer, csSDK_uint32 *ioUTF8StringBufferSizeInElements);
    prSuiteError (*AllocateFromUTF16)(const uint16_t *inUTF16String, PrSDKString *outSDKString);
    prSuiteError (*CopyToUTF16String)(const PrSDKString *inSDKString, uint16_t *outUTF16StringBuffer, csSDK_uint32 *ioUTF16StringBufferSizeInElements);
} PrSDKStringSuite;
#pragma pack(pop)
