use bevy::{
  asset::load_internal_asset,
  prelude::*,
  reflect::TypeUuid,
  render::{
    extract_resource::ExtractResource,
    render_resource::{Shader, ShaderType, UniformBuffer},
  },
};

#[derive(Default)]
pub struct ShaderUtils;

pub const SHADER_UTILS_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 1);

impl Plugin for ShaderUtils {
  fn build(&self, app: &mut App) {
    load_internal_asset!(
      app,
      SHADER_UTILS_HANDLE,
      "shaders/common.wgsl",
      Shader::from_wgsl
    );
  }
}

#[derive(Resource, Default)]
pub struct InfoBuffer {
  pub buffer: UniformBuffer<InfoUniform>,
}

#[derive(Default, Clone, Resource, ExtractResource, Reflect, ShaderType)]
#[reflect(Resource)]
pub struct InfoUniform {
  resolution: Vec2,
}
