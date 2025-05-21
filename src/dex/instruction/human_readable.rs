use crate::{dex::Dex, errors::TableIdxError};

use super::Instruction;

impl Instruction {
    pub fn to_human_readable(&self, dex: &Dex) -> Result<String, TableIdxError> {
        let mut out = String::from(self.opcode());

        macro_rules! pull_something {
            ($idx:expr, $table:expr, $err:ident) => {
                if let Some(something) = $table.get($idx as usize) {
                    something
                } else {
                    return Err(TableIdxError::$err($idx as usize));
                }
            };
        }

        macro_rules! pull_string {
            ($idx:expr) => {
                pull_something!($idx, dex.strings, String)
            };
        }

        macro_rules! pull_type {
            ($idx:expr) => {
                pull_something!($idx, dex.types, Type)
            };
        }

        macro_rules! pull_field {
            ($idx:expr) => {
                pull_something!($idx, dex.field_ids, FieldId)
            };
        }

        macro_rules! pull_method {
            ($idx:expr) => {
                pull_something!($idx, dex.method_ids, MethodId)
            };
        }

        macro_rules! pull_proto {
            ($idx:expr) => {
                pull_something!($idx, dex.proto_ids, ProtoId)
            };
        }

        let args = match self {
            Self::Nop | Self::ReturnVoid => String::new(),
            Self::Move { dst, src }
            | Self::MoveWide { dst, src }
            | Self::MoveObject { dst, src }
            | Self::ArrayLength { dst, array: src }
            | Self::NegInt { dst, src }
            | Self::NotInt { dst, src }
            | Self::NegLong { dst, src }
            | Self::NotLong { dst, src }
            | Self::NegFloat { dst, src }
            | Self::NegDouble { dst, src }
            | Self::IntToLong { dst, src }
            | Self::IntToFloat { dst, src }
            | Self::IntToDouble { dst, src }
            | Self::LongToInt { dst, src }
            | Self::LongToFloat { dst, src }
            | Self::LongToDouble { dst, src }
            | Self::FloatToInt { dst, src }
            | Self::FloatToLong { dst, src }
            | Self::FloatToDouble { dst, src }
            | Self::DoubleToInt { dst, src }
            | Self::DoubleToLong { dst, src }
            | Self::DoubleToFloat { dst, src }
            | Self::IntToByte { dst, src }
            | Self::IntToChar { dst, src }
            | Self::IntToShort { dst, src } => {
                format!("v{dst} v{src}")
            }
            Self::MoveFrom16 { dst, src }
            | Self::MoveWideFrom16 { dst, src }
            | Self::MoveObjectFrom16 { dst, src } => {
                format!("v{dst} v{src}")
            }
            Self::Move16 { dst, src }
            | Self::MoveWide16 { dst, src }
            | Self::MoveObject16 { dst, src } => {
                format!("v{dst} v{src}")
            }
            Self::MoveResult { dst }
            | Self::MoveResultWide { dst }
            | Self::MoveResultObject { dst }
            | Self::MoveException { dst }
            | Self::MonitorEnter { reference: dst }
            | Self::MonitorExit { reference: dst }
            | Self::Throw { exception: dst } => {
                format!("v{dst}")
            }
            Self::Return { value } | Self::ReturnWide { value } | Self::ReturnObject { value } => {
                format!("v{value}")
            }
            Self::Const4 { dst, value } => {
                format!("v{dst} {value}")
            }
            Self::Const16 { dst, value }
            | Self::ConstHigh16 { dst, value }
            | Self::ConstWide16 { dst, value }
            | Self::ConstWideHigh16 { dst, value } => {
                format!("v{dst} {value}")
            }
            Self::Const { dst, value } | Self::ConstWide32 { dst, value } => {
                format!("v{dst} {value}")
            }
            Self::ConstWide { dst, value } => {
                format!("v{dst} {value}")
            }

            Self::ConstString { dst, string_idx } => {
                let idx = *string_idx as usize;
                let string = pull_string!(idx);
                format!("v{dst} \"{string}\"")
            }
            Self::ConstStringJumbo { dst, string_idx } => {
                let idx = *string_idx as usize;
                let string = pull_string!(idx);
                format!("v{dst} \"{string}\"")
            }
            Self::ConstClass { dst, type_idx }
            | Self::CheckCast {
                reference: dst,
                type_idx,
            }
            | Self::NewInstance { dst, type_idx } => {
                let idx = *type_idx as usize;
                let t = pull_type!(idx);
                format!("v{dst} {t}")
            }
            Self::InstanceOf {
                dst,
                reference,
                type_idx,
            } => {
                let idx = *type_idx as usize;
                let t = pull_type!(idx);
                format!("v{dst} v{reference} {t}")
            }
            Self::NewArray {
                dst,
                size,
                type_idx,
            } => {
                let idx = *type_idx as usize;
                let t = pull_type!(idx);
                format!("v{dst} v{size} {t}")
            }
            Self::FilledNewArray {
                type_idx,
                args,
                arg_cnt,
            } => {
                let idx = *type_idx as usize;
                let mut args_str = String::new();
                for i in 0..*arg_cnt {
                    if let Some(arg) = args.get(i as usize) {
                        let local_arg = format!(" v{arg}");
                        args_str.push_str(&local_arg);
                    }
                }
                args_str = args_str.trim_start().to_string();
                let t = pull_type!(idx);
                format!("{args_str} {t}")
            }
            Self::FilledNewArrayRange {
                type_idx,
                first_arg,
                arg_cnt,
            } => {
                let idx = *type_idx as usize;
                let mut args_str = String::new();
                for i in 0..*arg_cnt {
                    let local_arg = format!(" v{}", *first_arg + i as u16);
                    args_str.push_str(&local_arg);
                }
                args_str = args_str.trim_start().to_string();
                let t = pull_type!(idx);
                format!("{args_str} {t}")
            }
            Self::FillArrayData { array, offset } => {
                format!("v{array} {offset}") // TODO: Use label?
            }
            Self::Goto { offset } => {
                format!("{offset}")
            }
            Self::Goto16 { offset } => {
                format!("{offset}")
            }
            Self::Goto32 { offset } => {
                format!("{offset}")
            }
            Self::PackedSwitch { value, offset } | Self::SparseSwitch { value, offset } => {
                format!("v{value} {offset}") // TODO: Use label?
            }
            Self::CmplFloat { dst, src_a, src_b }
            | Self::CmpgFloat { dst, src_a, src_b }
            | Self::CmplDouble { dst, src_a, src_b }
            | Self::CmpgDouble { dst, src_a, src_b }
            | Self::CmpLong { dst, src_a, src_b } => {
                format!("v{dst} v{src_a} v{src_b}")
            }
            Self::IfEq { a, b, offset }
            | Self::IfNe { a, b, offset }
            | Self::IfLt { a, b, offset }
            | Self::IfGe { a, b, offset }
            | Self::IfGt { a, b, offset }
            | Self::IfLe { a, b, offset } => {
                format!("v{a} v{b} {offset}") // TODO: Use label?
            }
            Self::IfEqz { a, offset }
            | Self::IfNez { a, offset }
            | Self::IfLtz { a, offset }
            | Self::IfGez { a, offset }
            | Self::IfGtz { a, offset }
            | Self::IfLez { a, offset } => {
                format!("v{a} {offset}") // TODO: Use label?
            }
            Self::Aget { src, array, index }
            | Self::AgetWide { src, array, index }
            | Self::AgetObject { src, array, index }
            | Self::AgetBoolean { src, array, index }
            | Self::AgetByte { src, array, index }
            | Self::AgetChar { src, array, index }
            | Self::AgetShort { src, array, index }
            | Self::Aput {
                dst: src,
                array,
                index,
            }
            | Self::AputWide {
                dst: src,
                array,
                index,
            }
            | Self::AputObject {
                dst: src,
                array,
                index,
            }
            | Self::AputBoolean {
                dst: src,
                array,
                index,
            }
            | Self::AputByte {
                dst: src,
                array,
                index,
            }
            | Self::AputChar {
                dst: src,
                array,
                index,
            }
            | Self::AputShort {
                dst: src,
                array,
                index,
            } => {
                format!("v{src} v{array} v{index}")
            }
            Self::Iget {
                src,
                object,
                field_idx,
            }
            | Self::IgetWide {
                src,
                object,
                field_idx,
            }
            | Self::IgetObject {
                src,
                object,
                field_idx,
            }
            | Self::IgetBoolean {
                src,
                object,
                field_idx,
            }
            | Self::IgetByte {
                src,
                object,
                field_idx,
            }
            | Self::IgetChar {
                src,
                object,
                field_idx,
            }
            | Self::IgetShort {
                src,
                object,
                field_idx,
            }
            | Self::Iput {
                dst: src,
                object,
                field_idx,
            }
            | Self::IputWide {
                dst: src,
                object,
                field_idx,
            }
            | Self::IputObject {
                dst: src,
                object,
                field_idx,
            }
            | Self::IputBoolean {
                dst: src,
                object,
                field_idx,
            }
            | Self::IputByte {
                dst: src,
                object,
                field_idx,
            }
            | Self::IputChar {
                dst: src,
                object,
                field_idx,
            }
            | Self::IputShort {
                dst: src,
                object,
                field_idx,
            } => {
                format!(
                    "v{src} v{object} {}",
                    pull_field!(*field_idx).to_human_readable(dex)?
                )
            }
            Self::Sget { src, field_idx }
            | Self::SgetWide { src, field_idx }
            | Self::SgetObject { src, field_idx }
            | Self::SgetBoolean { src, field_idx }
            | Self::SgetByte { src, field_idx }
            | Self::SgetChar { src, field_idx }
            | Self::SgetShort { src, field_idx }
            | Self::Sput {
                dst: src,
                field_idx,
            }
            | Self::SputWide {
                dst: src,
                field_idx,
            }
            | Self::SputObject {
                dst: src,
                field_idx,
            }
            | Self::SputBoolean {
                dst: src,
                field_idx,
            }
            | Self::SputByte {
                dst: src,
                field_idx,
            }
            | Self::SputChar {
                dst: src,
                field_idx,
            }
            | Self::SputShort {
                dst: src,
                field_idx,
            } => {
                format!("v{src} {}", pull_field!(*field_idx).to_human_readable(dex)?)
            }
            Self::InvokeVirtual {
                method_idx,
                args,
                arg_cnt,
            }
            | Self::InvokeSuper {
                method_idx,
                args,
                arg_cnt,
            }
            | Self::InvokeDirect {
                method_idx,
                args,
                arg_cnt,
            }
            | Self::InvokeStatic {
                method_idx,
                args,
                arg_cnt,
            }
            | Self::InvokeInterface {
                method_idx,
                args,
                arg_cnt,
            } => {
                let method = pull_method!(*method_idx);

                let mut args_str = String::new();
                for i in 0..*arg_cnt {
                    if let Some(arg) = args.get(i as usize) {
                        let local_arg = format!(" v{arg}");
                        args_str.push_str(&local_arg);
                    }
                }
                args_str = args_str.trim_start().to_string();
                format!("{args_str} {}", method.to_human_readable(dex)?)
            }
            Self::InvokeVirtualRange {
                method_idx,
                first_arg,
                arg_cnt,
            }
            | Self::InvokeSuperRange {
                method_idx,
                first_arg,
                arg_cnt,
            }
            | Self::InvokeDirectRange {
                method_idx,
                first_arg,
                arg_cnt,
            }
            | Self::InvokeStaticRange {
                method_idx,
                first_arg,
                arg_cnt,
            }
            | Self::InvokeInterfaceRange {
                method_idx,
                first_arg,
                arg_cnt,
            } => {
                let method = pull_method!(*method_idx);

                let mut args_str = String::new();
                for i in 0..*arg_cnt {
                    let local_arg = format!(" v{}", *first_arg + i as u16);
                    args_str.push_str(&local_arg);
                }
                args_str = args_str.trim_start().to_string();
                format!("{args_str} {}", method.to_human_readable(dex)?)
            }
            Self::AddInt { dst, src_a, src_b }
            | Self::SubInt { dst, src_a, src_b }
            | Self::MulInt { dst, src_a, src_b }
            | Self::DivInt { dst, src_a, src_b }
            | Self::RemInt { dst, src_a, src_b }
            | Self::AndInt { dst, src_a, src_b }
            | Self::OrInt { dst, src_a, src_b }
            | Self::XorInt { dst, src_a, src_b }
            | Self::ShlInt { dst, src_a, src_b }
            | Self::ShrInt { dst, src_a, src_b }
            | Self::UShrInt { dst, src_a, src_b }
            | Self::AddLong { dst, src_a, src_b }
            | Self::SubLong { dst, src_a, src_b }
            | Self::MulLong { dst, src_a, src_b }
            | Self::DivLong { dst, src_a, src_b }
            | Self::RemLong { dst, src_a, src_b }
            | Self::AndLong { dst, src_a, src_b }
            | Self::OrLong { dst, src_a, src_b }
            | Self::XorLong { dst, src_a, src_b }
            | Self::ShlLong { dst, src_a, src_b }
            | Self::ShrLong { dst, src_a, src_b }
            | Self::UShrLong { dst, src_a, src_b }
            | Self::AddFloat { dst, src_a, src_b }
            | Self::SubFloat { dst, src_a, src_b }
            | Self::MulFloat { dst, src_a, src_b }
            | Self::DivFloat { dst, src_a, src_b }
            | Self::RemFloat { dst, src_a, src_b }
            | Self::AddDouble { dst, src_a, src_b }
            | Self::SubDouble { dst, src_a, src_b }
            | Self::MulDouble { dst, src_a, src_b }
            | Self::DivDouble { dst, src_a, src_b }
            | Self::RemDouble { dst, src_a, src_b } => {
                format!("v{dst} v{src_a} v{src_b}")
            }
            Self::AddInt2Addr { dst, src }
            | Self::SubInt2Addr { dst, src }
            | Self::MulInt2Addr { dst, src }
            | Self::DivInt2Addr { dst, src }
            | Self::RemInt2Addr { dst, src }
            | Self::AndInt2Addr { dst, src }
            | Self::OrInt2Addr { dst, src }
            | Self::XorInt2Addr { dst, src }
            | Self::ShlInt2Addr { dst, src }
            | Self::ShrInt2Addr { dst, src }
            | Self::UShrInt2Addr { dst, src }
            | Self::AddLong2Addr { dst, src }
            | Self::SubLong2Addr { dst, src }
            | Self::MulLong2Addr { dst, src }
            | Self::DivLong2Addr { dst, src }
            | Self::RemLong2Addr { dst, src }
            | Self::AndLong2Addr { dst, src }
            | Self::OrLong2Addr { dst, src }
            | Self::XorLong2Addr { dst, src }
            | Self::ShlLong2Addr { dst, src }
            | Self::ShrLong2Addr { dst, src }
            | Self::UShrLong2Addr { dst, src }
            | Self::AddFloat2Addr { dst, src }
            | Self::SubFloat2Addr { dst, src }
            | Self::MulFloat2Addr { dst, src }
            | Self::DivFloat2Addr { dst, src }
            | Self::RemFloat2Addr { dst, src }
            | Self::AddDouble2Addr { dst, src }
            | Self::SubDouble2Addr { dst, src }
            | Self::MulDouble2Addr { dst, src }
            | Self::DivDouble2Addr { dst, src }
            | Self::RemDouble2Addr { dst, src } => {
                format!("v{dst} v{src}")
            }
            Self::AddIntLit16 { dst, src, value }
            | Self::RsubInt { dst, src, value }
            | Self::MulIntLit16 { dst, src, value }
            | Self::DivIntLit16 { dst, src, value }
            | Self::RemIntLit16 { dst, src, value }
            | Self::AndIntLit16 { dst, src, value }
            | Self::OrIntLit16 { dst, src, value }
            | Self::XorIntLit16 { dst, src, value } => {
                format!("v{dst} v{src} {value}")
            }
            Self::AddIntLit8 { dst, src, value }
            | Self::RsubIntLit8 { dst, src, value }
            | Self::MulIntLit8 { dst, src, value }
            | Self::DivIntLit8 { dst, src, value }
            | Self::RemIntLit8 { dst, src, value }
            | Self::AndIntLit8 { dst, src, value }
            | Self::OrIntLit8 { dst, src, value }
            | Self::XorIntLit8 { dst, src, value }
            | Self::ShlIntLit8 { dst, src, value }
            | Self::ShrIntLit8 { dst, src, value }
            | Self::UShrIntLit8 { dst, src, value } => {
                format!("v{dst} v{src} {value}")
            }
            Self::InvokePolymorphic {
                method_idx,
                proto_idx,
                args,
                arg_cnt,
            } => {
                let method = pull_method!(*method_idx);
                let proto = pull_proto!(*proto_idx);

                let mut args_str = String::new();
                for i in 0..*arg_cnt {
                    if let Some(arg) = args.get(i as usize) {
                        let local_arg = format!(" v{arg}");
                        args_str.push_str(&local_arg);
                    }
                }
                args_str = args_str.trim_start().to_string();

                format!(
                    "{args_str} {} {}",
                    method.to_human_readable(dex)?,
                    proto.to_human_readable(dex)?
                )
            }
            Self::InvokePolymorphicRange {
                method_idx,
                proto_idx,
                first_arg,
                arg_cnt,
            } => {
                let method = pull_method!(*method_idx);
                let proto = pull_proto!(*proto_idx);

                let mut args_str = String::new();
                for i in 0..*arg_cnt {
                    let local_arg = format!(" v{}", *first_arg + i as u16);
                    args_str.push_str(&local_arg);
                }
                args_str = args_str.trim_start().to_string();

                format!(
                    "{args_str} {} {}",
                    method.to_human_readable(dex)?,
                    proto.to_human_readable(dex)?
                )
            }
            Self::InvokeCustom {
                call_site_idx,
                args,
                arg_cnt,
            } => {
                let call_site = pull_something!(*call_site_idx, dex.call_site_items, CallSite);
                let mut args_str = String::new();
                for i in 0..*arg_cnt {
                    if let Some(arg) = args.get(i as usize) {
                        let local_arg = format!(" v{arg}");
                        args_str.push_str(&local_arg);
                    }
                }
                args_str = args_str.trim_start().to_string();
                format!("{args_str} {call_site:?}")
            }
            Self::InvokeCustomRange {
                call_site_idx,
                first_arg,
                arg_cnt,
            } => {
                let call_site = pull_method!(*call_site_idx);

                let mut args_str = String::new();
                for i in 0..*arg_cnt {
                    let local_arg = format!(" v{}", *first_arg + i as u16);
                    args_str.push_str(&local_arg);
                }
                args_str = args_str.trim_start().to_string();

                format!("{args_str} {}", call_site.to_human_readable(dex)?)
            }
            Self::ConstMethodHandle {
                dst,
                method_handle_idx,
            } => {
                let method_handle =
                    pull_something!(*method_handle_idx, dex.method_handles, MethodHandle);
                format!("v{dst} {method_handle:?}")
            }
            Self::ConstMethodType { dst, proto_idx } => {
                let proto = pull_proto!(*proto_idx);
                format!("v{dst} {}", proto.to_human_readable(dex)?)
            }
        };

        out.push(' ');
        out.push_str(&args);

        Ok(out)
    }
}
