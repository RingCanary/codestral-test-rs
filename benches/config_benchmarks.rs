use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use codestral_test_rs::config::Config;
use std::fs;
use std::path::Path;

fn bench_config_load_save(c: &mut Criterion) {
    let mut group = c.benchmark_group("Config Operations");
    
    // Benchmark default config creation
    group.bench_function("default_config", |b| {
        b.iter(|| Config::default())
    });
    
    // Prepare a test config file
    let test_file = "bench_config.toml";
    let config = Config::default();
    config.save_to_file(test_file).expect("Failed to save config");
    
    // Benchmark config loading
    group.bench_function("load_config", |b| {
        b.iter(|| Config::load_from_file(test_file))
    });
    
    // Benchmark config saving
    group.bench_function("save_config", |b| {
        b.iter(|| config.save_to_file(test_file))
    });
    
    // Clean up
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).expect("Failed to remove test file");
    }
    
    group.finish();
}

// Benchmark the config serialization with different models
fn bench_config_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("Config Serialization");
    
    for model_size in ["small", "medium", "large", "extra-large"] {
        let mut config = Config::default();
        config.code_model = format!("codestral-{}", model_size);
        config.chat_model = format!("mistral-{}", model_size);
        
        group.bench_with_input(
            BenchmarkId::new("serialize", model_size), 
            &config, 
            |b, config| {
                b.iter(|| toml::to_string_pretty(config))
            }
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_config_load_save, bench_config_serialization);
criterion_main!(benches);
