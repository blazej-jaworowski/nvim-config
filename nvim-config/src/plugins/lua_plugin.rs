use super::plugin::{Plugin, PluginError};
use crate::{
    Result,
    mlua::{self, FromLuaMulti, Function, IntoLuaMulti},
    nvim_helper::{lua_plugins::require_plugin, lua_value},
};
use std::{rc::Rc, result::Result as StdResult};

use std::marker::PhantomData;

pub struct LuaPlugin<'lua, A = (), R = ()>
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    name: String,

    setup_func: String,
    pre_setup: Option<Rc<dyn Fn() -> Result<A>>>,
    post_setup: Option<Rc<dyn Fn(R) -> Result<()>>>,

    _marker: PhantomData<&'lua ()>,
}

pub struct LuaPluginBuilder<'lua, A, R>
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    plugin: LuaPlugin<'lua, A, R>,
}

impl<'lua, A, R> LuaPluginBuilder<'lua, A, R>
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    pub fn setup_func(mut self, name: impl Into<String>) -> Self {
        self.plugin.name = name.into();
        self
    }

    pub fn pre_setup<F>(mut self, f: F) -> Self
    where
        F: Fn() -> Result<A> + 'static,
    {
        self.plugin.pre_setup = Some(Rc::new(f));
        self
    }

    pub fn post_setup<F>(mut self, f: F) -> Self
    where
        F: Fn(R) -> Result<()> + 'static,
    {
        self.plugin.post_setup = Some(Rc::new(f));
        self
    }

    pub fn build(self) -> LuaPlugin<'lua, A, R> {
        self.plugin
    }
}

enum LuaPluginSetupError {
    Plugin(PluginError),
    Lua(nvim_api_helper::mlua::Error),
    NvimApiHelper(nvim_api_helper::Error),
}

impl From<PluginError> for LuaPluginSetupError {
    fn from(value: PluginError) -> Self {
        LuaPluginSetupError::Plugin(value)
    }
}

impl From<nvim_api_helper::Error> for LuaPluginSetupError {
    fn from(value: nvim_api_helper::Error) -> Self {
        LuaPluginSetupError::NvimApiHelper(value)
    }
}

impl From<nvim_api_helper::mlua::Error> for LuaPluginSetupError {
    fn from(value: nvim_api_helper::mlua::Error) -> Self {
        LuaPluginSetupError::Lua(value)
    }
}

impl<'lua, A, R> LuaPlugin<'lua, A, R>
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    pub fn builder(name: impl Into<String>) -> LuaPluginBuilder<'lua, A, R> {
        LuaPluginBuilder {
            plugin: LuaPlugin {
                name: name.into(),
                setup_func: "setup".into(),
                pre_setup: None,
                post_setup: None,
                _marker: PhantomData,
            },
        }
    }
    fn lua_plugin_setup(&self) -> StdResult<(), LuaPluginSetupError> {
        if self.name.is_empty() {
            Err(PluginError::Other("Plugin name not provided".into()))?;
        }

        let lua = mlua::lua();

        let arg = if let Some(pre) = self.pre_setup.clone() {
            pre()?.into_lua_multi(lua)?
        } else {
            lua_value!([]).into_lua_multi(lua)?
        };

        let plugin_obj = match require_plugin(&self.name) {
            Ok(o) => o,
            Err(_) => Err(PluginError::NotInstalled(self.name.clone()))?,
        };
        let result: R = plugin_obj
            .get::<&str, Function>(&self.setup_func)?
            .call(arg)?;

        if let Some(post) = self.post_setup.clone() {
            post(result)?;
        }

        Ok(())
    }
}

impl<'lua, A, R> Plugin for LuaPlugin<'lua, A, R>
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    fn setup(&self) -> StdResult<(), PluginError> {
        match self.lua_plugin_setup() {
            Ok(o) => Ok(o),
            Err(LuaPluginSetupError::Plugin(e)) => Err(e),
            Err(LuaPluginSetupError::NvimApiHelper(e)) => Err(PluginError::Other(Box::new(e))),
            Err(LuaPluginSetupError::Lua(e)) => Err(PluginError::Other(Box::new(e))),
        }
    }
}

#[macro_export]
macro_rules! lua_plugin {
    ($name:expr) => {
        Box::new(LuaPlugin::<(), ()>::builder($name).build())
    };
    ($name:expr, $setup_arg:tt) => {
        Box::new(
            LuaPlugin::<_, ()>::builder($name)
                .pre_setup(|| Ok($crate::nvim_helper::lua_value!($setup_arg)))
                .build(),
        )
    };
}
