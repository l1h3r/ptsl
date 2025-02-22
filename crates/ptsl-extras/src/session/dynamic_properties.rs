use ptsl_client::error::Result;
use ptsl_protos::types::DpValueTypes;
use ptsl_protos::types::DynamicPropertyType;
use ptsl_protos::types::GetDynamicPropertiesGroup;
use ptsl_protos::types::GetDynamicPropertiesResponseBody;
use ptsl_protos::types::PropertyContainer;
use ptsl_protos::types::PropertyDescriptor;

use crate::utils::try_from_proto;

/// Dynamic property.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynamicProperties {
  kind: DynamicPropertyType,
  list: Vec<DynGroup>,
}

impl DynamicProperties {
  pub(crate) fn new(value: GetDynamicPropertiesResponseBody) -> Result<Self> {
    let list: Vec<DynGroup> = value
      .group_list
      .into_iter()
      .map(DynGroup::new)
      .try_collect()?;

    Ok(Self {
      kind: try_from_proto(value.property_type)?,
      list,
    })
  }

  /// Returns the dynamic property type.
  #[inline]
  pub const fn kind(&self) -> DynamicPropertyType {
    self.kind
  }

  /// Returns a list of dynamic property groups.
  #[inline]
  pub fn list(&self) -> &[DynGroup] {
    self.list.as_slice()
  }
}

// =============================================================================
// Dynamic Properties Group
// =============================================================================

/// Dynamic property group.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynGroup {
  containers: Vec<DynContainer>,
  descriptors: Vec<DynDescriptor>,
}

impl DynGroup {
  fn new(other: GetDynamicPropertiesGroup) -> Result<Self> {
    let containers: Vec<DynContainer> = other
      .key_list
      .into_iter()
      .map(DynContainer::new)
      .try_collect()?;

    let descriptors: Vec<DynDescriptor> = other
      .property_list
      .into_iter()
      .map(DynDescriptor::new)
      .try_collect()?;

    Ok(Self {
      containers,
      descriptors,
    })
  }

  /// Returns a list of dynamic property containers.
  #[inline]
  pub fn containers(&self) -> &[DynContainer] {
    self.containers.as_slice()
  }

  /// Returns a list of dynamic property descriptors.
  #[inline]
  pub fn descriptors(&self) -> &[DynDescriptor] {
    self.descriptors.as_slice()
  }
}

// =============================================================================
// Dynamic Property Container
// =============================================================================

/// Dynamic property container.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynContainer {
  container_name: String,
  value_type: DpValueTypes,
  value: String,
}

impl DynContainer {
  fn new(other: PropertyContainer) -> Result<Self> {
    Ok(Self {
      container_name: other.container_name,
      value_type: try_from_proto(other.r#type)?,
      value: other.value,
    })
  }

  /// Returns the dynamic property container name.
  #[inline]
  pub fn container_name(&self) -> &str {
    self.container_name.as_str()
  }

  /// Returns the dynamic property value type.
  #[inline]
  pub const fn value_type(&self) -> DpValueTypes {
    self.value_type
  }

  /// Returns the dynamic property value.
  #[inline]
  pub fn value(&self) -> &str {
    self.value.as_str()
  }
}

// =============================================================================
// Dynamic Property Descriptor
// =============================================================================

/// Dynamic property descriptor.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynDescriptor {
  name: String,
  value_type: DpValueTypes,
  object_type: String,
  required: bool,
  description: String,
  units: String,
  accepted_values: Vec<String>,
  max_value: String,
  min_value: String,
}

impl DynDescriptor {
  fn new(other: PropertyDescriptor) -> Result<Self> {
    Ok(Self {
      name: other.name,
      value_type: try_from_proto(other.value_type)?,
      object_type: other.object_type,
      required: other.required,
      description: other.description,
      units: other.units,
      accepted_values: other.accepted_values,
      max_value: other.max_value,
      min_value: other.min_value,
    })
  }

  /// Returns the dynamic property descriptor name.
  #[inline]
  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  /// Returns the dynamic property value type.
  #[inline]
  pub const fn value_type(&self) -> DpValueTypes {
    self.value_type
  }

  /// Returns the dynamic property object type.
  #[inline]
  pub fn object_type(&self) -> &str {
    self.object_type.as_str()
  }

  /// Returns `true `if the dynamic property is required.
  #[inline]
  pub const fn required(&self) -> bool {
    self.required
  }

  /// Returns the dynamic property description.
  #[inline]
  pub fn description(&self) -> &str {
    self.description.as_str()
  }

  /// Returns the dynamic property units.
  #[inline]
  pub fn units(&self) -> &str {
    self.units.as_str()
  }

  /// Returns the accepted values of the dynamic property.
  #[inline]
  pub fn accepted_values(&self) -> &[String] {
    self.accepted_values.as_slice()
  }

  /// Returns the dynamic property max value.
  #[inline]
  pub fn max_value(&self) -> &str {
    self.max_value.as_str()
  }

  /// Returns the dynamic property min value.
  #[inline]
  pub fn min_value(&self) -> &str {
    self.min_value.as_str()
  }
}
