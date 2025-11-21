#[derive(Default)]
pub(super) struct ParseState {
    pub(super) current_tag: String,
    pub(super) current_name: Option<String>,
    pub(super) namespace_stack: Vec<String>,
    pub(super) current_number_type: Option<String>, // For <number type="...">
    pub(super) template_params: Vec<crate::generator::parsing::ast::TemplateParam>, // For <template><param>
    pub(super) template_text: String, // Accumulated text for templates
    pub(super) in_template: bool, // Track if we're inside a <template> tag
    pub(super) in_array: bool, // Track if we're inside an <array> tag
    pub(super) array_type: Option<String>, // For <array type="...">
    pub(super) array_spec: Option<String>, // For <array spec="...">
    pub(super) array_items: Vec<String>, // Accumulated array items
    pub(super) pending_doc: Option<String>, // Documentation for the next resource
    pub(super) in_doc: bool, // Track if we're inside a <doc> tag
    pub(super) doc_text: String, // Accumulated text for doc
}
