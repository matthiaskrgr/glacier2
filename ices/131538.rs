#![feature(return_type_notation)]
#![feature(generic_associated_types_extended)]
#![feature(trivial_bounds)]

trait HealthCheck {
    async fn check<const N: usize>() -> bool;
}

async fn do_health_check_par<HC>()
where
    HealthCheck: HealthCheck<Send(): Send> + Send + 'static,
{
}
