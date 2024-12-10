use std::io::Result;

/// This is a stub for a custom service generator that generates a trait for each service.
/// It's built from the example in the prost-build documentation, and is a starting point for
/// generating service traits.
///
/// In order to progress in this direction, a few things generally need to happen:
///
/// - A syntax library and a pretty printer generally need to go into the build deps.
/// - This struct needs to use those somehow.
///
/// As of right now, that makes this a little more complex than is probably good for a rust
/// template, and it could create more confusion than it's worth. So, for now, this is a stub,
/// with some notes.
///
/// A possible next phase for this would be to push service kit templates into one part of this
/// project, and then bundle premade dependency crates in with as a monorepo. This would allow
/// us to plug in default behavior even when it's complicated, but keep the template simple to
/// use and understand.
///
struct ServiceTraitGenerator;

impl prost_build::ServiceGenerator for ServiceTraitGenerator {
    fn generate(&mut self, service: prost_build::Service, buf: &mut String) {
        // Generate a trait for the service.
        service.comments.append_with_indent(0, buf);
        buf.push_str(&format!("pub trait {} {{\n", &service.name));

        // Generate the service methods.
        for method in service.methods {
            method.comments.append_with_indent(1, buf);
            buf.push_str(&format!(
                "    fn {}(_: {}) -> {};\n",
                method.name, method.input_type, method.output_type
            ));
        }

        // Close out the trait.
        buf.push_str("}\n");
    }
}

fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();

    prost_build.service_generator(Box::new(ServiceTraitGenerator));
    prost_build.out_dir("protocol/output");

    prost_build.type_attribute(
        "protocol.services.HealthCheck",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );

    prost_build.type_attribute(
        "protocol.services.HealthCheckResponse",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );

    prost_build.include_file("protocol.rs");
    prost_build.compile_protos(&["protocol/services.proto"], &["protocol"])?;

    Ok(())
}
