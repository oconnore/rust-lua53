// The MIT License (MIT)
//
// Copyright (c) 2014 J.C. Moyer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

// corresponds to lua.h
use libc::{c_void, c_int, c_char, c_uchar, size_t};
use ffi::luaconf;
use std::ptr;

pub use super::glue::{LUA_VERSION_MAJOR, LUA_VERSION_MINOR, LUA_VERSION_NUM, LUA_VERSION_RELEASE};
pub use super::glue::{LUA_VERSION, LUA_RELEASE, LUA_COPYRIGHT, LUA_AUTHORS};

// LUA_SIGNATURE?

// option for multiple returns in 'lua_pcall' and 'lua_call'
pub const LUA_MULTRET: c_int = -1;

// pseudo-indices
pub const LUA_REGISTRYINDEX: c_int = luaconf::LUAI_FIRSTPSEUDOIDX;

#[inline(always)]
pub fn lua_upvalueindex(i: c_int) -> c_int {
  LUA_REGISTRYINDEX - i
}

// thread status
pub const LUA_OK: c_int = 0;
pub const LUA_YIELD: c_int = 1;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRGCMM: c_int = 5;
pub const LUA_ERRERR: c_int = 6;

pub type lua_State = c_void;

// basic types
pub const LUA_TNONE: c_int = -1;

pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TSTRING: c_int = 4;
pub const LUA_TTABLE: c_int = 5;
pub const LUA_TFUNCTION: c_int = 6;
pub const LUA_TUSERDATA: c_int = 7;
pub const LUA_TTHREAD: c_int = 8;

pub const LUA_NUMTAGS: c_int = 9;

// minimum stack available to a C function
pub const LUA_MINSTACK: c_int = 20;

// predefined values in the registry
pub const LUA_RIDX_MAINTHREAD: lua_Integer = 1;
pub const LUA_RIDX_GLOBALS: lua_Integer = 2;
pub const LUA_RIDX_LAST: lua_Integer = LUA_RIDX_GLOBALS;

// type of numbers in lua
pub type lua_Number = luaconf::LUA_NUMBER;

// type for integer functions
pub type lua_Integer = luaconf::LUA_INTEGER;

// unsigned integer type
pub type lua_Unsigned = luaconf::LUA_UNSIGNED;

// type for continuation-function contexts
pub type lua_Ctx = luaconf::LUA_CTXT;

// Type for C functions registered with Lua
pub type lua_CFunction = Option<fn(L: *mut lua_State) -> c_int>;

// Type for continuation functions
pub type lua_KFunction = Option<fn(L: *mut lua_State, status: c_int, ctx: lua_Ctx) -> c_int>;

// Type for functions that read/write blocks when loading/dumping Lua chunks
pub type lua_Reader = Option<fn(L: *mut lua_State, ud: *mut c_void, sz: *mut size_t) -> *const c_char>;
pub type lua_Writer = Option<fn(L: *mut lua_State, p: *const c_void, sz: size_t, ud: *mut c_void) -> c_int>;

// Type for memory-allocation functions
pub type lua_Alloc = Option<fn(ud: *mut c_void, ptr: *mut c_void, osize: size_t, nsize: size_t) -> *mut c_void>;

extern {
  // state manipulation
  pub fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State;
  pub fn lua_close(L: *mut lua_State);
  pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;

  pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;

  pub fn lua_version(L: *mut lua_State) -> *const lua_Number;

  // basic stack manipulation
  pub fn lua_absindex(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_gettop(L: *mut lua_State) -> c_int;
  pub fn lua_settop(L: *mut lua_State, idx: c_int);
  pub fn lua_pushvalue(L: *mut lua_State, idx: c_int);
  pub fn lua_rotate(L: *mut lua_State, idx: c_int, n: c_int);
  pub fn lua_copy(L: *mut lua_State, fromidx: c_int, toidx: c_int);
  pub fn lua_checkstack(L: *mut lua_State, sz: c_int) -> c_int;

  pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);

  // access functions (stack -> C)
  pub fn lua_isnumber(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_isstring(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_iscfunction(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_isinteger(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_typename(L: *mut lua_State, tp: c_int) -> *const c_char;

  pub fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;
  pub fn lua_tointegerx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Integer;
  pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_char;
  pub fn lua_rawlen(L: *mut lua_State, idx: c_int) -> size_t;
  pub fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> lua_CFunction;
  pub fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
  pub fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State;
  pub fn lua_topointer(L: *mut lua_State, idx: c_int) -> *const c_void;
}

// Comparison and arithmetic functions
pub const LUA_OPADD: c_int = 0;
pub const LUA_OPSUB: c_int = 1;
pub const LUA_OPMUL: c_int = 2;
pub const LUA_OPMOD: c_int = 3;
pub const LUA_OPPOW: c_int = 4;
pub const LUA_OPDIV: c_int = 5;
pub const LUA_OPIDIV: c_int = 6;
pub const LUA_OPBAND: c_int = 7;
pub const LUA_OPBOR: c_int = 8;
pub const LUA_OPBXOR: c_int = 9;
pub const LUA_OPSHL: c_int = 10;
pub const LUA_OPSHR: c_int = 11;
pub const LUA_OPUNM: c_int = 12;
pub const LUA_OPBNOT: c_int = 13;

extern {
  pub fn lua_arith(L: *mut lua_State, op: c_int);
}

pub const LUA_OPEQ: c_int = 0;
pub const LUA_OPLT: c_int = 1;
pub const LUA_OPLE: c_int = 2;

extern {
  pub fn lua_rawequal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;
  pub fn lua_compare(L: *mut lua_State, idx1: c_int, idx2: c_int, op: c_int) -> c_int;
}

// push functions (C -> stack)
extern {
  pub fn lua_pushnil(L: *mut lua_State);
  pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
  pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
  pub fn lua_pushlstring(L: *mut lua_State, s: *const c_char, l: size_t) -> *const c_char;
  pub fn lua_pushstring(L: *mut lua_State, s: *const c_char) -> *const c_char;
  // TODO: omitted:
  // lua_pushvfstring
  pub fn lua_pushfstring(L: *mut lua_State, fmt: *const c_char, ...) -> *const c_char;
  pub fn lua_pushcclosure(L: *mut lua_State, f: lua_CFunction, n: c_int);
  pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
  pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
  pub fn lua_pushthread(L: *mut lua_State) -> c_int;
}

// get functions (Lua -> stack)
extern {
  pub fn lua_getglobal(L: *mut lua_State, var: *const c_char) -> c_int;
  pub fn lua_gettable(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_getfield(L: *mut lua_State, idx: c_int, k: *const c_char) -> c_int;
  pub fn lua_rawget(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_rawgeti(L: *mut lua_State, idx: c_int, n: lua_Integer) -> c_int;
  pub fn lua_rawgetp(L: *mut lua_State, idx: c_int, p: *const c_void) -> c_int;

  pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
  pub fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut c_void;
  pub fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
  pub fn lua_getuservalue(L: *mut lua_State, idx: c_int) -> c_int;
}

// set functions (stack -> Lua)
extern {
  pub fn lua_setglobal(L: *mut lua_State, var: *const c_char);
  pub fn lua_settable(L: *mut lua_State, idx: c_int);
  pub fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char);
  pub fn lua_rawset(L: *mut lua_State, idx: c_int);
  pub fn lua_rawseti(L: *mut lua_State, idx: c_int, n: lua_Integer);
  pub fn lua_rawsetp(L: *mut lua_State, idx: c_int, p: *const c_void);
  pub fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
  pub fn lua_setuservalue(L: *mut lua_State, idx: c_int);
}

// 'load' and 'call' functions (load and run Lua code)
extern {
  pub fn lua_callk(L: *mut lua_State, nargs: c_int, nresults: c_int, ctx: lua_Ctx, k: lua_KFunction);
  pub fn lua_pcallk(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: lua_Ctx, k: lua_KFunction) -> c_int;
  pub fn lua_load(L: *mut lua_State, reader: lua_Reader, dt: *mut c_void, chunkname: *const c_char, mode: *const c_char) -> c_int;
  pub fn lua_dump(L: *mut lua_State, writer: lua_Writer, data: *mut c_void, strip: c_int) -> c_int;
}

#[inline(always)]
pub unsafe fn lua_call(L: *mut lua_State, n: c_int, r: c_int) {
  lua_callk(L, n, r, 0, None)
}

#[inline(always)]
pub unsafe fn lua_pcall(L: *mut lua_State, n: c_int, r: c_int, f: c_int) -> c_int {
  lua_pcallk(L, n, r, f, 0, None)
}

// coroutine functions
extern {
  pub fn lua_yieldk(L: *mut lua_State, nresults: c_int, ctx: lua_Ctx, k: lua_KFunction) -> c_int;
  pub fn lua_resume(L: *mut lua_State, from: *mut lua_State, narg: c_int) -> c_int;
  pub fn lua_status(L: *mut lua_State) -> c_int;
  pub fn lua_isyieldable(L: *mut lua_State) -> c_int;
}

#[inline(always)]
pub unsafe fn lua_yield(L: *mut lua_State, n: c_int) -> c_int {
  lua_yieldk(L, n, 0, None)
}

// garbage-collection function and options
pub const LUA_GCSTOP: c_int = 0;
pub const LUA_GCRESTART: c_int = 1;
pub const LUA_GCCOLLECT: c_int = 2;
pub const LUA_GCCOUNT: c_int = 3;
pub const LUA_GCCOUNTB: c_int = 4;
pub const LUA_GCSTEP: c_int = 5;
pub const LUA_GCSETPAUSE: c_int = 6;
pub const LUA_GCSETSTEPMUL: c_int = 7;
pub const LUA_GCISRUNNING: c_int = 9;

extern {
  pub fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;
}

// miscellaneous functions
extern {
  pub fn lua_error(L: *mut lua_State) -> c_int;
  pub fn lua_next(L: *mut lua_State, idx: c_int) -> c_int;
  pub fn lua_concat(L: *mut lua_State, n: c_int);
  pub fn lua_len(L: *mut lua_State, idx: c_int);
  pub fn lua_strtonum(L: *mut lua_State, s: *const c_char) -> size_t;
  pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc;
  pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut c_void);
}

// some useful macros
// here, implemented as Rust functions
#[inline(always)]
pub unsafe fn lua_getextraspace(L: *mut lua_State) -> *mut c_void {
  L.offset(-super::glue::LUA_EXTRASPACE as int) as *mut c_void
}

#[inline(always)]
pub unsafe fn lua_tonumber(L: *mut lua_State, i: c_int) -> lua_Number {
  lua_tonumberx(L, i, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_tointeger(L: *mut lua_State, i: c_int) -> lua_Integer {
  lua_tointegerx(L, i, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
  lua_settop(L, -n - 1)
}

#[inline(always)]
pub unsafe fn lua_newtable(L: *mut lua_State) {
  lua_createtable(L, 0, 0)
}

#[inline(always)]
pub unsafe fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) {
  lua_pushcfunction(L, f);
  lua_setglobal(L, n)
}

#[inline(always)]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
  lua_pushcclosure(L, f, 0)
}

#[inline(always)]
pub unsafe fn lua_isfunction(L: *mut lua_State, n: c_int) -> c_int {
  (lua_type(L, n) == LUA_TFUNCTION) as c_int
}

#[inline(always)]
pub unsafe fn lua_istable(L: *mut lua_State, n: c_int) -> c_int {
  (lua_type(L, n) == LUA_TTABLE) as c_int
}

#[inline(always)]
pub unsafe fn lua_islightuserdata(L: *mut lua_State, n: c_int) -> c_int {
  (lua_type(L, n) == LUA_TLIGHTUSERDATA) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> c_int {
  (lua_type(L, n) == LUA_TNIL) as c_int
}

#[inline(always)]
pub unsafe fn lua_isboolean(L: *mut lua_State, n: c_int) -> c_int {
  (lua_type(L, n) == LUA_TBOOLEAN) as c_int
}

#[inline(always)]
pub unsafe fn lua_isthread(L: *mut lua_State, n: c_int) -> c_int {
  (lua_type(L, n) == LUA_TTHREAD) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnone(L: *mut lua_State, n: c_int) -> c_int {
  (lua_type(L, n) == LUA_TNONE) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> c_int {
  (lua_type(L, n) <= 0) as c_int
}

// TODO: Test
#[inline(always)]
pub unsafe fn lua_pushliteral(L: *mut lua_State, s: &'static str) -> *const c_char {
  s.with_c_str(|c_str| lua_pushlstring(L, c_str, s.len() as size_t))
}

#[inline(always)]
pub unsafe fn lua_pushglobaltable(L: *mut lua_State) -> c_int {
  lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS)
}

#[inline(always)]
pub unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char {
  lua_tolstring(L, i, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_insert(L: *mut lua_State, idx: c_int) {
  lua_rotate(L, idx, 1)
}

#[inline(always)]
pub unsafe fn lua_remove(L: *mut lua_State, idx: c_int) {
  lua_rotate(L, idx, -1);
  lua_pop(L, 1)
}

#[inline(always)]
pub unsafe fn lua_replace(L: *mut lua_State, idx: c_int) {
  lua_copy(L, -1, idx);
  lua_pop(L, 1)
}

// Debug API
// Event codes
pub const LUA_HOOKCALL: c_int = 0;
pub const LUA_HOOKRET: c_int = 1;
pub const LUA_HOOKLINE: c_int = 2;
pub const LUA_HOOKCOUNT: c_int = 3;
pub const LUA_HOOKTAILCALL: c_int = 4;

// Event masks
pub const LUA_MASKCALL: c_int = 1 << (LUA_HOOKCALL as uint);
pub const LUA_MASKRET: c_int = 1 << (LUA_HOOKRET as uint);
pub const LUA_MASKLINE: c_int = 1 << (LUA_HOOKLINE as uint);
pub const LUA_MASKCOUNT: c_int = 1 << (LUA_HOOKCOUNT as uint);

// Functions to be called by the debugger in specific events
pub type lua_Hook = Option<fn(L: *mut lua_State, ar: *mut lua_Debug)>;

extern {
  pub fn lua_getstack(L: *mut lua_State, level: c_int, ar: *mut lua_Debug) -> c_int;
  pub fn lua_getinfo(L: *mut lua_State, what: *const c_char, ar: *mut lua_Debug) -> c_int;
  pub fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_char;
  pub fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_char;
  pub fn lua_getupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;
  pub fn lua_setupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;

  pub fn lua_upvalueid(L: *mut lua_State, fidx: c_int, n: c_int) -> *mut c_void;
  pub fn lua_upvaluejoin(L: *mut lua_State, fidx1: c_int, n1: c_int, fidx2: c_int, n2: c_int);

  pub fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: c_int, count: c_int);
  pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
  pub fn lua_gethookmask(L: *mut lua_State) -> c_int;
  pub fn lua_gethookcount(L: *mut lua_State) -> c_int;
}

#[repr(C)]
pub struct lua_Debug {
  pub event: c_int,
  pub name: *const c_char,
  pub namewhat: *const c_char,
  pub what: *const c_char,
  pub source: *const c_char,
  pub currentline: c_int,
  pub linedefined: c_int,
  pub lastlinedefined: c_int,
  pub nups: c_uchar,
  pub nparams: c_uchar,
  pub isvararg: c_char,
  pub istailcall: c_char,
  pub short_src: [c_char, ..luaconf::LUA_IDSIZE as uint],
  // lua.h mentions this is for private use
  i_ci: *mut c_void,
}
