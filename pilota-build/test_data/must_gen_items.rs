pub mod must_gen_items {
    #![allow(warnings, clippy::all)]

    pub mod must_gen_items {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub a: ::std::option::Option<i32>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.a.as_ref() {
                    protocol.write_i32_field(1, *value)?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut a = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            offset += protocol.field_stop_len();
                            break;
                        } else {
                            offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                a = Some(protocol.read_i32()?);
                            }
                            _ => {
                                offset += protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let data = Self { a };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut a = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                a = Some(protocol.read_i32().await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let data = Self { a };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| protocol.i32_field_len(Some(1), *value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
    }
}
