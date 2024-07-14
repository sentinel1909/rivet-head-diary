// rivet-head-diary site
// src/lib/components/input.rs

// dependencies
use yew::{function_component, Html, html, NodeRef, Properties};

// struct type to represent the properties for a generic set of form inputs
#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
  pub label: String,
  pub field_type: String,
  pub name: String,
  pub input_node_ref: NodeRef,
}

// function to create a generic component to represent the input fields of a form
#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
  let InputFieldProps {
    label, field_type, name, input_node_ref,
  } = props;

  html! {
    <label for="cautious-input">
      { label }
      <input 
        type={field_type.clone()}
        name={name.clone()}
        ref={input_node_ref.clone()}
      />
    </label>
  }
}

