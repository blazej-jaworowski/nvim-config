use crate::{
    Result,
    mlua::{
        self,
        Function,
        IntoLuaMulti, FromLuaMulti,
    },
    nvim,
    nvim_helper::{
        lua_value,
        lua_plugins::require_plugin,
    },
};

use std::{
    marker::PhantomData,
    rc::Rc,
};

pub struct LuaPlugin<'lua, A = (), R = ()>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{
    name: String,

    setup_func: Option<String>,
    pre_func: Option<Rc<dyn Fn() -> Result<A>>>,
    post_func: Option<Rc<dyn Fn(R) -> Result<()>>>,

    _marker: PhantomData<&'lua ()>,
}

impl<'lua, A, R> Default for LuaPlugin<'lua, A, R>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{

    fn default() -> Self {
        LuaPlugin {
            name: String::new(),
            setup_func: None,
            pre_func: None,
            post_func: None,
            _marker: PhantomData,
        }
    }

}

impl<'lua, A, R> Clone for LuaPlugin<'lua, A, R>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{

    fn clone(&self) -> Self {
        LuaPlugin {
            name: self.name.clone(),
            setup_func: self.setup_func.clone(),
            pre_func: self.pre_func.clone(),
            post_func: self.post_func.clone(),
            _marker: PhantomData,
        }
    }

}

impl<'lua, A, R> LuaPlugin<'lua, A, R>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{

    pub fn builder() -> LuaPluginBuilder<'lua, A, R> {
        LuaPluginBuilder::new()
    }

    pub fn setup(&self) -> Result<()> {
        let lua = mlua::lua();

        let arg = if let Some(pre) = &self.pre_func {
            let a = pre().map_err(|e| {
                nvim::print!("Failed to run pre_func for {}: {e}", self.name);
                e
            })?;
            a.into_lua_multi(lua)?
        } else {
            lua_value!([]).into_lua_multi(lua)?
        };

        let result: R = if let Some(f) = &self.setup_func {
            require_plugin(&self.name)?
                .get::<&str, Function>(f)?
                .call(arg)?
        } else {
            if self.post_func.is_some() {
                nvim::print!("No setup_func, but post_func given. Ignoring it.");
            }
            return Ok(());
        };

        if let Some(post) = &self.post_func {
            (*post)(result).map_err(|e| {
                nvim::print!("Failed to run post_func for {}: {e}", self.name);
                e
            })?;
        }

        Ok(())
    }

}

pub struct LuaPluginBuilder<'lua, A, R>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{
    plugin: LuaPlugin<'lua, A, R>,
}

impl<'lua, A, R> Clone for LuaPluginBuilder<'lua, A, R>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{

    fn clone(&self) -> Self {
        LuaPluginBuilder {
            plugin: self.plugin.clone(),
        }
    }

}

#[allow(dead_code)]
impl<'lua, A, R> LuaPluginBuilder<'lua, A, R>
where
A: IntoLuaMulti<'lua>,
R: FromLuaMulti<'lua>,
{

    pub fn new() -> Self {
        LuaPluginBuilder {
            plugin: LuaPlugin::default(),
        }
    }

    pub fn build(self) -> LuaPlugin<'lua, A, R> {
        if self.plugin.name.is_empty() {
            nvim::print!("WARNING: Creating a plugin without a name");
        }
        self.plugin.clone()
    }

    pub fn name(&self, name: &str) -> Self {
        let mut next = self.clone();
        next.plugin.name = name.to_string();
        next
    }

    pub fn setup_func(&self, setup_func: &str) -> Self {
        let mut next = self.clone();
        next.plugin.setup_func = Some(setup_func.to_string());
        next
    }

    pub fn pre_func(&self, pre_func: Rc<dyn Fn() -> Result<A>>) -> Self {
        let mut next = self.clone();
        next.plugin.pre_func = Some(pre_func);
        next
    }

    pub fn post_func(&self, post_func: Rc<dyn Fn(R) -> Result<()>>) -> Self {
        let mut next = self.clone();
        next.plugin.post_func = Some(post_func);
        next
    }

}

#[allow(dead_code)]
pub fn setup_plugins<'lua>(plugins: impl IntoIterator<Item = LuaPlugin<'lua>>)
{
    for plugin in plugins {
        if let Err(e) = plugin.setup() {
            nvim::print!("Failed to setup '{}': {e}", plugin.name);
        }
    }
}
