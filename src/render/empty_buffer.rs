use bevy::{
    ecs::{
        resource::Resource,
        system::{Res, ResMut},
    },
    render::{
        render_resource::{BindingResource, Buffer, BufferDescriptor, BufferUsages},
        renderer::RenderDevice,
    },
};

/// A resource serving as a general purpose "empty" buffer, allowing us to use it as a
/// stand in for times we don't have a usable binding (such as an empty array).
#[derive(Resource, Default)]
pub struct EmptyBuffer {
    pub buffer: Option<Buffer>,
}

impl EmptyBuffer {
    pub fn binding(&self) -> Option<BindingResource> {
        self.buffer
            .as_ref()
            .map(|buffer| BindingResource::Buffer(buffer.as_entire_buffer_binding()))
    }

    pub fn fill_buffer(&mut self, render_device: &RenderDevice) {
        // GpuArrayBuffer uniform buffers are 4096 bytes in size when using WebGl2.
        //
        // On platforms that support dynamic storage buffers, we just need something big
        // enough to "hold" one item.
        let size = if render_device.limits().max_storage_buffers_per_shader_stage == 0 {
            4096
        } else {
            64
        };
        if self.buffer.is_none() {
            self.buffer = Some(render_device.create_buffer(&BufferDescriptor {
                label: "empty-buffer".into(),
                size,
                usage: BufferUsages::COPY_DST | BufferUsages::STORAGE | BufferUsages::UNIFORM,
                mapped_at_creation: false,
            }));
        }
    }
}

pub fn prepare_empty_buffer(
    mut empty_buffer: ResMut<EmptyBuffer>,
    render_device: Res<RenderDevice>,
) {
    empty_buffer.fill_buffer(&render_device);
}
