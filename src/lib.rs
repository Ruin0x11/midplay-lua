extern crate midplay;
extern crate mlua;

use std::fmt::Display;
use mlua::prelude::*;
use midplay::{native, generic};

type OpResult = (bool, Option<String>);

fn to_op_result<T, E: Display>(result: Result<T, E>) -> OpResult {
    match result {
        Ok(_) => (true, None),
        Err(mes) => (false, Some(format!("{}", mes))),
    }
}

fn native_play_midi(_: &Lua, path: String) -> LuaResult<OpResult> {
    Ok(to_op_result(native::play_midi(&path)))
}

fn native_is_midi_playing(_: &Lua, (): ()) -> LuaResult<bool> {
    Ok(native::is_midi_playing())
}

fn native_stop_midi(_: &Lua, (): ()) -> LuaResult<OpResult> {
    Ok(to_op_result(native::stop_midi()))
}

fn generic_get_ports(lua: &Lua, (): ()) -> LuaResult<LuaTable> {
    let ports = generic::get_ports().map_err(|e| e.to_lua_err())?;

    let result = lua.create_table()?;

    for (i, port) in ports.into_iter().enumerate() {
        let table = lua.create_table()?;
        table.set("index", port.index as i64)?;
        table.set("name", port.name)?;
        result.set(i + 1, table)?;
    }

    Ok(result)
}

fn generic_play_midi(_: &Lua, (path, port): (String, i64)) -> LuaResult<OpResult> {
    let ports = generic::get_ports().map_err(|e| e.to_lua_err())?;
    let port = ports.get(port as usize);
    if port.is_none() {
        return Err("invalid port number".to_lua_err());
    }

    Ok(to_op_result(generic::play_midi(&path, port.unwrap())))
}

fn generic_is_midi_playing(_: &Lua, _: ()) -> LuaResult<bool> {
    Ok(generic::is_midi_playing())
}

fn generic_stop_midi(_: &Lua, _: ()) -> LuaResult<OpResult> {
    Ok(to_op_result(generic::stop_midi()))
}

#[mlua::lua_module]
fn midplay_lua(lua: &Lua) -> LuaResult<LuaTable> {
    let native = lua.create_table()?;
    native.set("play_midi", lua.create_function(native_play_midi)?)?;
    native.set("is_midi_playing", lua.create_function(native_is_midi_playing)?)?;
    native.set("stop_midi", lua.create_function(native_stop_midi)?)?;

    let generic = lua.create_table()?;
    generic.set("get_ports", lua.create_function(generic_get_ports)?)?;
    generic.set("play_midi", lua.create_function(generic_play_midi)?)?;
    generic.set("is_midi_playing", lua.create_function(generic_is_midi_playing)?)?;
    generic.set("stop_midi", lua.create_function(generic_stop_midi)?)?;

    let exports = lua.create_table()?;
    exports.set("native", native)?;
    exports.set("generic", generic)?;
    Ok(exports)
}
