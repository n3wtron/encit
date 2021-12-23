#[cfg(feature = "ui")]
use actix_web_static_files::NpmBuild;

fn main() -> std::io::Result<()> {
    #[cfg(feature = "ui")]
    {
        NpmBuild::new("web")
            .executable("yarn")
            .install()?
            .run("build")?
            .target("web/dist/web")
            .to_resource_dir()
            .build()
    }
    #[cfg(not(feature = "ui"))]
    Ok(())
}
