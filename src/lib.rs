use obs_wrapper::{
    log::Logger,
    obs_register_module,
    obs_string,
    prelude::*,
    source::*,
};

struct MyPlugin {
    context: ModuleContext,
}

#[derive(Debug)]
struct Data;

impl Sourceable for MyPlugin {
    fn get_id() -> ObsString {
        obs_string!("my_plugin_source")
    }

    fn get_type() -> SourceType {
        SourceType::FILTER
    }
}


impl Module for MyPlugin {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        let _ = Logger::new().with_promote_debug(true).init();

        let source = load_context
            .create_source_builder::<MyPlugin, Data>()
            .enable_get_name()
            .enable_create()
            .build();

        // Tell OBS about the source so that it will show it.
        load_context.register_source(source);

        // Nothing could have gone wrong, so return true.
        true
    }

    fn description() -> ObsString {
        obs_string!("This is a test")
    }

    fn name() -> ObsString {
        obs_string!("My plugin")
    }

    fn author() -> ObsString {
        obs_string!("Me :)")
    }
}

impl GetNameSource<Data> for MyPlugin {
    fn get_name() -> ObsString {
        obs_string!("Testing")
    }
}

impl CreatableSource<Data> for MyPlugin {
    fn create(_create: &mut CreatableSourceContext<Data>, _source: SourceContext) -> Data {
        use log::*;
        trace!("Trace");
        debug!("Debug");
        info!("Info");
        warn!("Warn");
        error!("Error");

        Data
    }
}

obs_register_module!(MyPlugin);
