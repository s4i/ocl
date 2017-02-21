//! An `OpenCL` command queue.

use std;
use std::ops::{Deref, DerefMut};
use core::error::{Result as OclResult};
use core::{self, CommandQueue as CommandQueueCore, Context as ContextCore,
    CommandQueueInfo, CommandQueueInfoResult, OpenclVersion, CommandQueueProperties};
use standard::{Context, Device};

/// A command queue which manages all actions taken on kernels, buffers, and
/// images.
///
///
//
// TODO: Consider implementing a constructor which accepts a DeviceIdCore and
// creates a context and queue from it.
//
// [NOTE]: The `context_obj_core` and `device` fields could easily be removed
// and their values retrieved via `::info`. It's a matter of whether or not
// the two extra pointers reference count increment/decrement is worth the
// tradeoff of slower information access. It's a balance between how often the
// queue is cloned and how often it's queried for info. Right now the balance
// is heavily in favor of persisting the context and device here due to the
// frequency of access from various enqueue functions on kernel, buffer, et
// al. If the enqueuers stored their own context it could be removed here but
// it doesn't seem worth worrying about.
//
#[derive(Clone, Debug)]
pub struct Queue {
    obj_core: CommandQueueCore,
    context_obj_core: ContextCore,
    device: Device,
    device_version: OpenclVersion,
}

impl Queue {
    /// Returns a new Queue on the device specified by `device`.
    pub fn new(context: &Context, device: Device, properties: Option<CommandQueueProperties>)
            -> OclResult<Queue> {
        let obj_core = try!(core::create_command_queue(context, &device, properties));
        let device_version = try!(device.version());

        Ok(Queue {
            obj_core: obj_core,
            context_obj_core: context.core().clone(),
            device: device,
            device_version: device_version,
        })
    }

    /// Issues all previously queued OpenCL commands to the device.
    pub fn flush(&self) -> OclResult<()> {
        core::flush(&self.obj_core)
    }

    /// Blocks until all commands in this queue have completed before returning.
    pub fn finish(&self) -> OclResult<()> {
        core::finish(&self.obj_core)
    }

    /// Returns a reference to the core pointer wrapper, usable by functions in
    /// the `core` module.
    #[deprecated(since="0.13.0", note="Use `::core` instead.")]
    pub fn core_as_ref(&self) -> &CommandQueueCore {
        &self.obj_core
    }

    /// Returns a reference to the core pointer wrapper, usable by functions in
    /// the `core` module.
    #[inline]
    pub fn core(&self) -> &CommandQueueCore {
        &self.obj_core
    }

    /// Returns a reference to the core pointer wrapper of the context
    /// associated with this queue, usable by functions in the `core` module.
    pub fn context_core(&self) -> &ContextCore {
        &self.context_obj_core
    }

    /// Returns the `OpenCL` device associated with this queue.
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Returns the cached device version.
    pub fn device_version(&self) -> OpenclVersion {
        self.device_version
    }

    /// Returns info about this queue.
    pub fn info(&self, info_kind: CommandQueueInfo) -> CommandQueueInfoResult {
        core::get_command_queue_info(&self.obj_core, info_kind)
    }

    fn fmt_info(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Queue")
            .field("Context", &self.info(CommandQueueInfo::Context))
            .field("Device", &self.info(CommandQueueInfo::Device))
            .field("ReferenceCount", &self.info(CommandQueueInfo::ReferenceCount))
            .field("Properties", &self.info(CommandQueueInfo::Properties))
            .finish()
    }
}

impl std::fmt::Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_info(f)
    }
}

impl AsRef<CommandQueueCore> for Queue {
    fn as_ref(&self) -> &CommandQueueCore {
        &self.obj_core
    }
}

impl Deref for Queue {
    type Target = CommandQueueCore;

    fn deref(&self) -> &CommandQueueCore {
        &self.obj_core
    }
}

impl DerefMut for Queue {
    fn deref_mut(&mut self) -> &mut CommandQueueCore {
        &mut self.obj_core
    }
}
