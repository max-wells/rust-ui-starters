use leptos::*;

#[server(ReadMdxFile, "/api")]
pub async fn read_mdx_file(path: String) -> Result<String, ServerFnError> {
    std::fs::read_to_string(path)
        .map_err(|e| ServerFnError::ServerError(format!("Failed to read MDX file: {}", e)))
}
