use rlua::{Error as LuaError, Lua};
use sumdir;

pub fn gen_checksum(_lua: &Lua, path: String) -> Result<String, LuaError> {
    sumdir::dir_hash(&path[..])
        .map_err(LuaError::external)
}