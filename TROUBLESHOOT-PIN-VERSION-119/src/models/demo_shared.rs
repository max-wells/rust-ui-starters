use crate::components::layouts::layout_demos_routes_core::DemoCoreTrait;
use crate::components::layouts::layout_demos_routes_extensions::DemoExtensionsTrait;
use crate::components::layouts::layout_demos_routes_hooks::DemoHooksTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct DemoCore {
    pub name: &'static str,
    pub path_url: &'static str,
    pub path_mdx: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}

impl DemoCoreTrait for DemoCore {
    fn name(&self) -> &'static str {
        self.name
    }

    fn path_url(&self) -> &'static str {
        self.path_url
    }

    fn path_mdx(&self) -> &'static str {
        self.path_mdx
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn tags(&self) -> &'static [&'static str] {
        self.tags
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

//
//

#[derive(Clone, Debug, PartialEq)]
pub struct DemoHooks {
    pub name: &'static str,
    pub path_url: &'static str,
    pub path_mdx: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}

impl DemoHooksTrait for DemoHooks {
    fn name(&self) -> &'static str {
        self.name
    }

    fn path_url(&self) -> &'static str {
        self.path_url
    }

    fn path_mdx(&self) -> &'static str {
        self.path_mdx
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn tags(&self) -> &'static [&'static str] {
        self.tags
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

//
//

#[derive(Clone, Debug, PartialEq)]
pub struct DemoExtensions {
    pub name: &'static str,
    pub path_url: &'static str,
    pub path_mdx: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}

impl DemoExtensionsTrait for DemoExtensions {
    fn name(&self) -> &'static str {
        self.name
    }

    fn path_url(&self) -> &'static str {
        self.path_url
    }

    fn path_mdx(&self) -> &'static str {
        self.path_mdx
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn tags(&self) -> &'static [&'static str] {
        self.tags
    }
}
