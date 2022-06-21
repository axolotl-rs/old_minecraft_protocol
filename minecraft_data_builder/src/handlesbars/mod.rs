use convert_case::{Case, Casing};
use handlebars::{Context, Handlebars, Helper, JsonValue, no_escape, Output, Renderable, RenderContext, RenderError};
use serde::{Serialize};
use serde_json::json;
use crate::Templates;


#[derive(Debug, Serialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub switch: Option<String>,

}

#[derive(Debug, Serialize, Clone)]
pub struct SwitchVariant {
    pub name: String,
    pub requirement: String,

    pub void: bool,
    pub fields: Option<Vec<Field>>,
    pub single: Option<String>,
}
#[derive(Debug, Serialize, Clone)]
pub enum SwitchVariantType{
    Void,
    Container(Vec<Field>),
    Single(String)
}

#[derive(Debug, Clone)]
pub enum GenerateType {
    Full {
        content_name: String,
        fields: Vec<Field>,
        wrap_in_mod: bool,
        children: Vec<GenerateType>,
    },
    SwitchEnum {
        content_name: String,
         compare_to: String,
         compare_to_type: String,
        generic_compare: bool,
        variants: Vec<SwitchVariant>,
        wrap_in_mod: bool,
        children: Vec<GenerateType>,
    },
}

impl GenerateType {
    pub fn generate(self, registry: &handlebars::Handlebars) -> Result<String, handlebars::RenderError> {
        match self {
            GenerateType::Full { wrap_in_mod, content_name, fields,children } => {
                let children = children.into_iter().map(|child| child.generate(registry)).collect::<Result<Vec<_>, _>>()?;

                registry.render("ContentTemplate", &json!({
                    "wrap_in_mod": wrap_in_mod,
                    "content_name": content_name,
                    "fields": fields,
                    "children": children,
                    "as_str": false,
                }))
            }
            GenerateType::SwitchEnum{wrap_in_mod, content_name, variants,children,compare_to,compare_to_type,generic_compare} => {
                let template = if generic_compare{
                    "GenericSwitch"
                }else{
                    "SwitchEnum"
                };
                let children = children.into_iter().map(|child| child.generate(registry)).collect::<Result<Vec<_>, _>>()?;
                registry.render(template, &json!({
                    "wrap_in_mod": wrap_in_mod,
                    "content_name": content_name,
                    "variants": variants,
                    "children": children,
                    "compare_to": compare_to,
                    "compare_to_type": compare_to_type
                }))
            }
        }
    }
}

pub fn create_handlebars()->handlebars::Handlebars<'static>{
    let mut registry = handlebars::Handlebars::new();
    registry.register_escape_fn(no_escape);
    let cow = Templates::get("ContentTemplate.rs.hbs").unwrap().data;
    let template = String::from_utf8_lossy(cow.as_ref());
    registry.register_template_string("ContentTemplate", template).unwrap();
    let cow = Templates::get("SwitchEnum.rs.hbs").unwrap().data;
    let template = String::from_utf8_lossy(cow.as_ref());
    registry.register_template_string("SwitchEnum", template).unwrap();
    let cow = Templates::get("GenericSwitch.rs.hbs").unwrap().data;
    let template = String::from_utf8_lossy(cow.as_ref());
    registry.register_template_string("GenericSwitch", template).unwrap();
    registry.register_helper("case", Box::new(case_helper));
    registry.register_helper("exists", Box::new(exists_helper));
    registry
}
fn case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    if h.params().len() !=2{
        return Err(RenderError::new("Invalid number of arguments"));
    }
    let content = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderError::new(
            "Param 0 needs to be content",
        ))?.to_string();
    let case = h
        .param(1)
        .as_ref()
        .and_then(|v| v.value().as_str())
        .map(|arr| arr.to_string())
        .ok_or(RenderError::new(
            "Param Requires a Case {snake, camel}",
        ))?;
    match case.as_str() {
        "camel" => {
            out.write(content.to_case(Case::UpperCamel).as_str())?
        }
        "snake" => {
            out.write(content.to_case(Case::Snake).as_str())?
        }
        _ => {
            // This is how I punish the user for not selecting the correct case
            out.write(content.to_case(Case::Alternating).as_str())?
        }
    }
    Ok(())
}

fn exists_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    handlebars: &'reg Handlebars,
    context: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0)
                          .ok_or_else(|| RenderError::new("Param not found for helper \"exists\""))?;

    let exists = param.value() != &JsonValue::Null;

    let tmpl = if exists {
        h.template()
    } else {
        h.inverse()
    };

    match tmpl {
        Some( t) =>{
            let string = t.renders(handlebars, context, rc)?;
            out.write(string.as_str())?;
            Ok(())
        },
        None => Ok(()),
    }
}

