extern crate jvmti;
use jvmti::{
    agent::Agent,
    config::Config,
    context::static_context,
    native::{JavaVMPtr, MutString, ReturnValue, VoidPtr},
    options::Options,
};

use jvmti::util::stringify;

mod agent;

use crate::agent::{
    on_class_file_load, on_garbage_collection_finish, on_garbage_collection_start, on_method_entry,
    on_method_exit, on_monitor_contended_enter, on_monitor_contended_entered, on_monitor_wait,
    on_monitor_waited, on_object_alloc, on_object_free, on_thread_end, on_thread_start,
};

///
/// `Agent_OnLoad` is the actual entry point of the agent code and it is called by the
/// Java Virtual Machine directly.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn Agent_OnLoad(
    vm: JavaVMPtr,
    options: MutString,
    reserved: VoidPtr,
) -> ReturnValue {
    let options = Options::parse(stringify(options));
    println!("Starting up as {}", options.agent_id);

    if let Some(config) = Config::read_config() {
        println!("Setting configuration");
        static_context().set_config(config);
    }

    let mut agent = Agent::new(vm);

    agent.on_garbage_collection_start(Some(on_garbage_collection_start));
    agent.on_garbage_collection_finish(Some(on_garbage_collection_finish));
    agent.on_vm_object_alloc(Some(on_object_alloc));
    agent.on_vm_object_free(Some(on_object_free));
    agent.on_class_file_load(Some(on_class_file_load));
    agent.on_method_entry(Some(on_method_entry));
    agent.on_method_exit(Some(on_method_exit));
    agent.on_thread_start(Some(on_thread_start));
    agent.on_thread_end(Some(on_thread_end));
    agent.on_monitor_wait(Some(on_monitor_wait));
    agent.on_monitor_waited(Some(on_monitor_waited));
    agent.on_monitor_contended_enter(Some(on_monitor_contended_enter));
    agent.on_monitor_contended_entered(Some(on_monitor_contended_entered));
    agent.on_class_file_load(Some(on_class_file_load));

    agent.update();

    return 0;
}

///
/// `Agent_OnUnload` is the exit point of the agent code. It is called when the JVM has finished
/// running and the virtual machine is unloading the agent from memory before shutting down.
/// Note: this method is also called when the JVM crashes due to an internal error.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn Agent_OnUnload(vm: JavaVMPtr) {}
