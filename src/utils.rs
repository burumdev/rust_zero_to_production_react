use actix_web::error::ErrorInternalServerError;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub fn e500<E>(e: E) -> actix_web::Error
where
    E: std::fmt::Debug + std::fmt::Display + 'static,
{
    ErrorInternalServerError(e)
}
