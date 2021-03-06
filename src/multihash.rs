#![allow(dead_code)]

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HashType {
    SHA1,
    SHA2_256,
    SHA256,
    SHA2_512_256,
    SHA2_512_512,
    SHA512,
    BLAKE2B_256,
    BLAKE2B_512,
    BLAKE2S_128,
    BLAKE2S_256,
}

pub type HashBytes = (HashType, Box<[u8]>);

pub fn hash_type_to_string(hash_type: HashType) -> String {
    use self::HashType::*;

    match hash_type {
        SHA1 => String::from("SHA1"),
        SHA2_256 | SHA256 => String::from("SHA256"),
        SHA2_512_256 => String::from("SHA2-512-256"),
        SHA2_512_512 | SHA512 => String::from("SHA512"),
        BLAKE2B_256 => String::from("BLAKE2b-256"),
        BLAKE2B_512 => String::from("BLAKE2b-512"),
        BLAKE2S_128 => String::from("BLAKE2s-128"),
        BLAKE2S_256 => String::from("BLAKE2s-256"),
    }
}

pub fn string_to_hash_type(string: &str) -> Result<HashType, ()> {
    let string = string.to_lowercase();

    use self::HashType::*;

    let hash_type = match string.as_str() {
        "sha1" => Ok(SHA1),
        "sha2-256" | "sha256" => Ok(SHA256),
        "sha2-512-256" => Ok(SHA2_512_256),
        "sha2-512-512" | "sha512" => Ok(SHA512),
        "blake2b-256" => Ok(BLAKE2B_256),
        "blake2b-512" => Ok(BLAKE2B_512),
        "blake2s-128" => Ok(BLAKE2S_128),
        "blake2s-256" => Ok(BLAKE2S_256),
        _ => Err(()),
    };

    match hash_type {
        Ok(hs) => {
            if hash::hash_type_is_supported(hs) {
                Ok(hs)
            } else {
                Err(())
            }
        }
        Err(()) => Err(()),
    }
}

pub fn hash_bytes_to_bytes(hash_bytes: &HashBytes, buffer: &mut [u8]) {
    let param = specs::Param::new(hash_bytes.0);
    let digest_bytes = &hash_bytes.1;
    for i in 0..param.hash_func_type.len() {
        buffer[i] = param.hash_func_type[i];
    }

    buffer[param.hash_func_type.len()] = param.digest_length;

    let offset = param.hash_func_type.len() + 1;

    for i in 0..param.digest_length as usize {
        buffer[i + offset] = digest_bytes[i];
    }
}

pub fn hash_bytes_into_bytes(hash_bytes: &HashBytes) -> Box<[u8]> {
    let param = specs::Param::new(hash_bytes.0);
    let mut buffer = vec![0; param.total_length()].into_boxed_slice();

    hash_bytes_to_bytes(hash_bytes, &mut buffer);
    buffer
}

pub mod specs {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub struct Param {
        pub hash_func_type: &'static [u8],
        pub digest_length: u8,
    }

    macro_rules! param {
        (
            $func_type:ident; $len:expr
        ) => {
            Param {
                hash_func_type: &$func_type,
                digest_length: $len,
            }
        };
    }

    static SHA1_HFT: [u8; 1] = [0x11];
    static SHA256_HFT: [u8; 1] = [0x12];
    static SHA512_HFT: [u8; 1] = [0x13];
    static BLAKE2B_256_HFT: [u8; 2] = [0xb2, 0x20];
    static BLAKE2B_512_HFT: [u8; 2] = [0xb2, 0x40];
    static BLAKE2S_128_HFT: [u8; 2] = [0xb2, 0x50];
    static BLAKE2S_256_HFT: [u8; 2] = [0xb2, 0x60];

    pub static SHA1_PARAM: Param = param!(SHA1_HFT; 0x14);
    pub static SHA256_PARAM: Param = param!(SHA256_HFT; 0x20);
    pub static SHA2_512_256_PARAM: Param = param!(SHA512_HFT; 0x20);
    pub static SHA512_PARAM: Param = param!(SHA512_HFT; 0x40);
    pub static BLAKE2B_256_PARAM: Param = param!(BLAKE2B_256_HFT; 0x20);
    pub static BLAKE2B_512_PARAM: Param = param!(BLAKE2B_512_HFT; 0x40);
    pub static BLAKE2S_128_PARAM: Param = param!(BLAKE2S_128_HFT; 0x10);
    pub static BLAKE2S_256_PARAM: Param = param!(BLAKE2S_256_HFT; 0x20);

    impl Param {
        pub fn new(hash_type: HashType) -> Param {
            use super::HashType::*;
            match hash_type {
                SHA1 => SHA1_PARAM,
                SHA2_256 | SHA256 => SHA256_PARAM,
                SHA2_512_256 => SHA2_512_256_PARAM,
                SHA2_512_512 | SHA512 => SHA512_PARAM,
                BLAKE2B_256 => BLAKE2B_256_PARAM,
                BLAKE2B_512 => BLAKE2B_512_PARAM,
                BLAKE2S_128 => BLAKE2S_128_PARAM,
                BLAKE2S_256 => BLAKE2S_256_PARAM,
            }
        }

        pub fn total_length(&self) -> usize {
            self.hash_func_type.len() + 1 + self.digest_length as usize
        }
    }
}

pub mod hash {
    use super::*;

    use blake2::{VarBlake2b, VarBlake2s};

    #[derive(Clone, Debug)]
    pub struct Ctx {
        ctx: _Ctx,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Debug)]
    enum _Ctx {
        SHA1(sha1::Sha1),
        SHA256(sha2::Sha256),
        SHA512(sha2::Sha512),
        BLAKE2B_256(VarBlake2b),
        BLAKE2B_512(VarBlake2b),
        BLAKE2S_128(VarBlake2s),
        BLAKE2S_256(VarBlake2s),
    }

    pub fn hash_type_is_supported(hash_type: HashType) -> bool {
        match Ctx::new(hash_type) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    impl Ctx {
        pub fn new(hash_type: HashType) -> Result<Ctx, ()> {
            let ctx = match hash_type {
                HashType::SHA1 => {
                    use sha1::Digest;
                    Some(_Ctx::SHA1(sha1::Sha1::new()))
                }
                HashType::SHA2_256 | HashType::SHA256 => {
                    use sha2::Digest;
                    Some(_Ctx::SHA256(sha2::Sha256::new()))
                }
                HashType::SHA2_512_256 => None,
                HashType::SHA2_512_512 | HashType::SHA512 => {
                    use sha2::Digest;
                    Some(_Ctx::SHA512(sha2::Sha512::new()))
                }
                HashType::BLAKE2B_256 => {
                    use blake2::digest::VariableOutput;
                    Some(_Ctx::BLAKE2B_256(
                        VarBlake2b::new(specs::Param::new(hash_type).digest_length as usize)
                            .unwrap(),
                    ))
                }
                HashType::BLAKE2B_512 => {
                    use blake2::digest::VariableOutput;
                    Some(_Ctx::BLAKE2B_512(
                        VarBlake2b::new(specs::Param::new(hash_type).digest_length as usize)
                            .unwrap(),
                    ))
                }
                HashType::BLAKE2S_128 => {
                    use blake2::digest::VariableOutput;
                    Some(_Ctx::BLAKE2S_128(
                        VarBlake2s::new(specs::Param::new(hash_type).digest_length as usize)
                            .unwrap(),
                    ))
                }
                HashType::BLAKE2S_256 => {
                    use blake2::digest::VariableOutput;
                    Some(_Ctx::BLAKE2S_256(
                        VarBlake2s::new(specs::Param::new(hash_type).digest_length as usize)
                            .unwrap(),
                    ))
                }
            };
            match ctx {
                Some(ctx) => Ok(Ctx { ctx }),
                None => Err(()),
            }
        }

        pub fn hash_type(&self) -> HashType {
            match self.ctx {
                _Ctx::SHA1(_) => HashType::SHA1,
                _Ctx::SHA256(_) => HashType::SHA256,
                _Ctx::SHA512(_) => HashType::SHA512,
                _Ctx::BLAKE2B_256(_) => HashType::BLAKE2B_256,
                _Ctx::BLAKE2B_512(_) => HashType::BLAKE2B_512,
                _Ctx::BLAKE2S_128(_) => HashType::BLAKE2S_128,
                _Ctx::BLAKE2S_256(_) => HashType::BLAKE2S_256,
            }
        }

        pub fn update(&mut self, data: &[u8]) {
            match self.ctx {
                _Ctx::SHA1(ref mut ctx) => {
                    use sha1::Digest;
                    ctx.input(data)
                }
                _Ctx::SHA256(ref mut ctx) => {
                    use sha2::Digest;
                    ctx.input(data)
                }
                _Ctx::SHA512(ref mut ctx) => {
                    use sha2::Digest;
                    ctx.input(data)
                }
                _Ctx::BLAKE2B_256(ref mut ctx) => {
                    use blake2::digest::Input;
                    ctx.input(data);
                }
                _Ctx::BLAKE2B_512(ref mut ctx) => {
                    use blake2::digest::Input;
                    ctx.input(data);
                }
                _Ctx::BLAKE2S_128(ref mut ctx) => {
                    use blake2::digest::Input;
                    ctx.input(data);
                }
                _Ctx::BLAKE2S_256(ref mut ctx) => {
                    use blake2::digest::Input;
                    ctx.input(data);
                }
            }
        }

        pub fn finish_to_bytes(self, hashval: &mut [u8]) {
            match self.ctx {
                _Ctx::SHA1(ctx) => {
                    use sha1::Digest;
                    hashval.copy_from_slice(&ctx.result())
                }
                _Ctx::SHA256(ctx) => {
                    use sha2::Digest;
                    hashval.copy_from_slice(&ctx.result())
                }
                _Ctx::SHA512(ctx) => {
                    use sha2::Digest;
                    hashval.copy_from_slice(&ctx.result())
                }
                _Ctx::BLAKE2B_256(ctx) => {
                    use blake2::digest::VariableOutput;
                    hashval.copy_from_slice(&ctx.vec_result())
                }
                _Ctx::BLAKE2B_512(ctx) => {
                    use blake2::digest::VariableOutput;
                    hashval.copy_from_slice(&ctx.vec_result())
                }
                _Ctx::BLAKE2S_128(ctx) => {
                    use blake2::digest::VariableOutput;
                    hashval.copy_from_slice(&ctx.vec_result())
                }
                _Ctx::BLAKE2S_256(ctx) => {
                    use blake2::digest::VariableOutput;
                    hashval.copy_from_slice(&ctx.vec_result())
                }
            }
        }

        pub fn finish_into_bytes(self) -> Box<[u8]> {
            let hash_type = self.hash_type();
            let param = specs::Param::new(hash_type);
            let digest_len = param.digest_length;
            let mut hashval = vec![0; digest_len as usize].into_boxed_slice();
            self.finish_to_bytes(&mut hashval);
            hashval
        }

        pub fn finish_to_hash_bytes(self, hash_bytes: &mut HashBytes) {
            hash_bytes.0 = self.hash_type();
            self.finish_to_bytes(&mut hash_bytes.1);
        }

        pub fn finish_into_hash_bytes(self) -> HashBytes {
            (self.hash_type(), self.finish_into_bytes())
        }
    }
}

pub mod parsers {
    use super::super::misc_utils;
    use super::specs;
    use super::{HashBytes, HashType};

    macro_rules! make_hash_parser_w_len {
        (
            $name:ident, $ht:path, $param:path
        ) => {
            named!(
                $name<HashBytes>,
                do_parse!(
                    _total_len: tag!(&[$param.total_length() as u8])
                        >> _id: tag!($param.hash_func_type)
                        >> _n: tag!(&[$param.digest_length])
                        >> res: take!($param.digest_length)
                        >> (($ht, misc_utils::slice_to_boxed(res)))
                )
            );
        };
    }

    make_hash_parser_w_len!(sha1_w_len_p, HashType::SHA1, specs::SHA1_PARAM);
    make_hash_parser_w_len!(sha256_w_len_p, HashType::SHA256, specs::SHA256_PARAM);
    make_hash_parser_w_len!(
        sha2_512_256_w_len_p,
        HashType::SHA2_512_256,
        specs::SHA2_512_256_PARAM
    );
    make_hash_parser_w_len!(sha512_w_len_p, HashType::SHA512, specs::SHA512_PARAM);
    make_hash_parser_w_len!(
        blake2b_256_w_len_p,
        HashType::BLAKE2B_256,
        specs::BLAKE2B_256_PARAM
    );
    make_hash_parser_w_len!(
        blake2b_512_w_len_p,
        HashType::BLAKE2B_512,
        specs::BLAKE2B_512_PARAM
    );
    make_hash_parser_w_len!(
        blake2s_128_w_len_p,
        HashType::BLAKE2S_128,
        specs::BLAKE2S_128_PARAM
    );
    make_hash_parser_w_len!(
        blake2s_256_w_len_p,
        HashType::BLAKE2S_256,
        specs::BLAKE2S_256_PARAM
    );

    named!(pub multihash_w_len_p <HashBytes>,
           alt!(
               complete!(sha1_w_len_p)
                   | complete!(sha256_w_len_p)
                   | complete!(sha2_512_256_w_len_p)
                   | complete!(sha512_w_len_p)
                   | complete!(blake2b_256_w_len_p)
                   | complete!(blake2b_512_w_len_p)
                   | complete!(blake2s_128_w_len_p)
                   | complete!(blake2s_256_w_len_p)
           )
    );
}
