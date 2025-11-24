#  Moseiik

Moseiik est un projet qui permet de générer des mosaïques d'images à partir d'un corpus de vignettes (appelées tiles) et d'une image de référence (appelée target). Le projet inclut des optimisations **SIMD** (SSE2 pour x86 et NEON pour ARM) pour le calcul de la distance L1, ainsi que la parallélisation via multithreading.

Ce projet est préparé en 3 parties : implémentation des tests unitaires et d'intégration, Docker et Intégration Continue (CI).

### Tests Unitaires (`src/main.rs`)

| Fonction | Logique de Test Résumée |
| :--- | :--- |
| **`unit_test_generic`** | Vérifie le calcul correct de la distance L1. Pour ce faire, nous avons chargé une image simple, tile_0, et on a comparé la distance avec elle-même. Étant donné qu'il s'agit de la même image, la distance doit être nulle, et nous utilisons donc cette condition comme critère de succès pour le test. |
| **`unit_test_x86`** | S'assurer que la fonction **SSE2** (x86) produit un résultat **identique** à la fonction `l1_generic`. |
| **`unit_test_aarch64`** | S'assurer que la fonction **NEON** (ARM) produit un résultat **identique** à la fonction `l1_generic`. |
| **`unit_test_prepare_tiles`** | Vérifie que le chargement et le redimensionnement sont corrects : **nombre** de vignettes attendu et **dimensions** et `--tile-size`. |
| **`unit_test_prepare_target`** | Vérifie que "target" est correctement redimensionnée : les **dimensions finales** doivent être des multiples de `tile-size`. |

### Tests d'Intégration (`tests/`)

| Test | Logique de Test Résumée |
| :--- | :--- |
| **`test_generic`** (Intégration Fonctionnelle) | Exécute la fonction principale `compute_mosaic` et compare l'image générée (`out.png`) à une image de **vérité terrain** (`assets/ground-truth-kit.png`), pour une architecture generique. |
| **`test_x86`** (Intégration Fonctionnelle) | Exécute la fonction principale `compute_mosaic` et compare l'image générée (`out.png`) à une image de **vérité terrain** (`assets/ground-truth-kit.png`), pour l'architecture x86. |
| **`test_aarch64`** (Intégration Fonctionnelle) | Exécute la fonction principale `compute_mosaic` et compare l'image générée (`out.png`) à une image de **vérité terrain** (`assets/ground-truth-kit.png`), pour l'architecture ARM64. |

-----

## 🐳 Docker

Nous utilisons Docker pour gérer les dépendances et faciliter les tests multi-architectures via la CI.

### Dockerfile

  * Le conteneur est basé sur l'image légère **`rust:1.85-slim`**.
  * Tous les fichiers sources et de tests sont copiés dans le répertoire de travail `/app`.
  * L'instruction **`ENTRYPOINT`** est utilisée pour définir la commande par défaut : `cargo test --release --`. Cela permet d'exécuter directement les tests lors du lancement du conteneur (`docker run`) et de passer facilement des arguments additionnels à la commande `cargo test`.

-----

## CI: Intégration Continue avec GitHub Actions

Notre pipeline de CI utilise GitHub Actions pour exécuter les tests automatiquement sur chaque `push` ou `pull_request`. Il est optimisé pour le multi-architecture grâce à l'utilisation d'une matrix et de QEMU.

### Fichier `.github/workflows/ci_moseiik.yaml`

Pour pouvoir cibler les architectures **`amd64`** et **`arm64`** dans un seul *job* le workflow utilise une matrice.

#### 1\. Configuration Multi-Architecture (QEMU et Buildx)

Les actions `docker/setup-qemu-action` et `docker/setup-buildx-action` sont utilisées pour préparer l'environnement. Cela permet à la machine virtuelle GitHub Actions (qui est nativement x86) d'**émuler l'architecture ARM64** (via QEMU) et de construire des images multi-architectures.

#### 2\. Téléchargement des Images de Test

**Observation :** Afin de ne pas alourdir le dépôt GitHub avec une base de données de 400 images de test, on a choisi de télécharger les vignettes dynamiquement pendant l'exécution de la CI:

```yaml
- name: Download test images
  run: |
    # ... installation de curl et unzip ...
    curl -L "https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip" -o moseiik_test_images.zip
    unzip -o moseiik_test_images.zip -d assets
    # ...
```

Cette étape assure que les tests disposent de la base complète sans que celle-ci ne soit versionnée dans le dépôt.

#### 3\. Construction et Exécution

L'action `docker/build-push-action` construit l'image Docker pour l'architecture spécifiée dans la matrice (`linux/${{ matrix.arch }}`). Enfin, la commande **`docker run`** exécute le conteneur, ce qui lance automatiquement les tests grâce à l'**`ENTRYPOINT`** défini dans le Dockerfile.