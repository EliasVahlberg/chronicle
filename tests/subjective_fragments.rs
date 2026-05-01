//! Phase 4: Subjective fragment queries (issue #4).

use std::path::Path;
use chronicle::graph::Chronicle;
use chronicle::model::*;

fn load() -> Chronicle {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/world");
    Chronicle::from_directory(&path).unwrap()
}

// ── accounts_by source ─────────────────────────────────────

#[test]
fn accounts_by_kaine_durgan() {
    let c = load();
    let accounts = c.accounts_by("kaine_durgan");
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].id, "kaine_durgan_siege_journal");
    assert_eq!(accounts[0].fidelity, Fidelity::Biased);
}

#[test]
fn accounts_by_jorik_vane() {
    let c = load();
    let accounts = c.accounts_by("jorik_vane");
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].id, "jorik_vane_schism_account");
    assert_eq!(accounts[0].fidelity, Fidelity::Partial);
}

#[test]
fn accounts_by_nonexistent_source() {
    let c = load();
    assert!(c.accounts_by("nobody").is_empty());
}

// ── Multiple accounts of the same event ────────────────────

#[test]
fn two_accounts_of_siege_of_silica() {
    let c = load();
    let accounts = c.accounts_of("siege_of_silica");
    assert_eq!(accounts.len(), 2);

    let sources: Vec<&str> = accounts.iter().map(|a| a.source.as_str()).collect();
    assert!(sources.contains(&"kaine_durgan"));
    assert!(sources.contains(&"jorik_vane"));
}

#[test]
fn accounts_of_siege_have_different_fidelity() {
    let c = load();
    let accounts = c.accounts_of("siege_of_silica");
    let fidelities: Vec<&Fidelity> = accounts.iter().map(|a| &a.fidelity).collect();
    assert!(fidelities.contains(&&Fidelity::Biased));
    assert!(fidelities.contains(&&Fidelity::Partial));
}

// ── Filtering accounts by fidelity ─────────────────────────

#[test]
fn filter_accounts_by_fidelity() {
    let c = load();
    let accounts = c.accounts_of("siege_of_silica");

    let biased: Vec<_> = accounts.iter().filter(|a| a.fidelity == Fidelity::Biased).collect();
    assert_eq!(biased.len(), 1);
    assert_eq!(biased[0].source, "kaine_durgan");

    let partial: Vec<_> = accounts.iter().filter(|a| a.fidelity == Fidelity::Partial).collect();
    assert_eq!(partial.len(), 1);
    assert_eq!(partial[0].source, "jorik_vane");
}

// ── Divergence: what entities do different accounts mention? ─

#[test]
fn accounts_mention_different_entities() {
    let c = load();
    let accounts = c.accounts_of("siege_of_silica");

    let kaine_account = accounts.iter().find(|a| a.source == "kaine_durgan").unwrap();
    let jorik_account = accounts.iter().find(|a| a.source == "jorik_vane").unwrap();

    let kaine_mentions: Vec<String> = chronicle::graph::parse_references(&kaine_account.text);
    let jorik_mentions: Vec<String> = chronicle::graph::parse_references(&jorik_account.text);

    // Both mention silica and mirror_order
    assert!(kaine_mentions.contains(&"silica".to_owned()));
    assert!(jorik_mentions.contains(&"silica".to_owned()));
    assert!(kaine_mentions.contains(&"mirror_order".to_owned()));
    assert!(jorik_mentions.contains(&"mirror_order".to_owned()));

    // Jorik mentions actors/events that Kaine doesn't
    assert!(jorik_mentions.contains(&"kaine_durgan".to_owned()));
    assert!(jorik_mentions.contains(&"brother_halix".to_owned()));
    assert!(jorik_mentions.contains(&"iron_covenant".to_owned()));
    assert!(!kaine_mentions.contains(&"brother_halix".to_owned()));
}

// ── Broader coverage: jorik covers multiple events ─────────

#[test]
fn jorik_account_covers_multiple_events() {
    let c = load();
    let accounts = c.accounts_by("jorik_vane");
    let jorik = &accounts[0];
    assert_eq!(jorik.event_refs.len(), 3);
    assert!(jorik.event_refs.contains(&"siege_of_silica".to_owned()));
    assert!(jorik.event_refs.contains(&"battle_of_broken_glass".to_owned()));
    assert!(jorik.event_refs.contains(&"purist_uprising".to_owned()));
}

// ── jorik_vane is no longer an orphan ──────────────────────

#[test]
fn jorik_vane_no_longer_orphan() {
    let c = load();
    let report = c.validate();
    let jorik_orphan = report.warnings.iter().any(|w| {
        matches!(w, chronicle::validation::ValidationWarning::OrphanEntity { entity_id }
            if entity_id == "jorik_vane")
    });
    assert!(!jorik_orphan, "jorik_vane should not be orphaned now that he has an account");
}
