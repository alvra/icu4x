// @generated
/// Implement `DataProvider<BidiClassValueToLongNameV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_long_linear_bc_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_TO_LONG_LINEAR_BC_V1: &'static <icu::properties::provider::BidiClassValueToLongNameV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\r\0\x1A\0)\0;\0N\0[\0k\0~\0\x8F\0\x9A\0\xA7\0\xBE\0\xD4\0\xE1\0\xF8\0\x0E\x01$\x013\x01C\x01W\x01l\x01\x81\x01Left_To_RightRight_To_LeftEuropean_NumberEuropean_SeparatorEuropean_TerminatorArabic_NumberCommon_SeparatorParagraph_SeparatorSegment_SeparatorWhite_SpaceOther_NeutralLeft_To_Right_EmbeddingLeft_To_Right_OverrideArabic_LetterRight_To_Left_EmbeddingRight_To_Left_OverridePop_Directional_FormatNonspacing_MarkBoundary_NeutralFirst_Strong_IsolateLeft_To_Right_IsolateRight_To_Left_IsolatePop_Directional_Isolate") } };
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::properties::provider::BidiClassValueToLongNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::BidiClassValueToLongNameV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_TO_LONG_LINEAR_BC_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::BidiClassValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
