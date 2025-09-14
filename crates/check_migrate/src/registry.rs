use crate::analyzers::MigrationCategory;
use check_analyze::RegistryVisitor;
use check_json_syntax::JsonLanguage;

pub fn visit_migration_registry<V: RegistryVisitor<JsonLanguage>>(registry: &mut V) {
    registry.record_category::<MigrationCategory>();
}
