// @generated
/// Implement `DataProvider<SentenceBreakNameToValueV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_sb_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_FROM_SB_V1: &'static <icu::properties::provider::SentenceBreakNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::names::PropertyValueNameToEnumMapV1 {
                map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x1C\0\0\0\0\0\x02\0\x07\0\t\0\x0E\0\x10\0\x12\0\x18\0\x1A\0 \0\"\0$\0&\0+\0-\x004\0;\0@\0B\0K\0M\0P\0R\0T\0V\0[\0]\0b\0ATATermCLCloseCREXExtendFOFormatLELFLOLowerNUNumericOLetterOtherSCSContinueSESepSPSpSTSTermUPUpperXX") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\x01\0\x02\0\x02\0\x0B\0\x0C\0\x0C\0\x03\0\x03\0\x06\0\r\0\x04\0\x04\0\x05\0\x05\0\x06\0\0\0\x0E\0\x0E\0\x07\0\x07\0\x08\0\x08\0\t\0\t\0\n\0\n\0\0\0") })
                },
            };
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::properties::provider::SentenceBreakNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakNameToValueV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_FROM_SB_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::SentenceBreakNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
