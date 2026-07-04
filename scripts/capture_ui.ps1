<#
.SYNOPSIS
    Headless screenshot harness for Eclipse Heart.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives it through the env-var capture hook
    (ECLIPSE_HEART_CAPTURE_*) provided by macroquad_toolkit::capture in
    src/main.rs. Scenes are seeded via Game::prepare_capture_screen.

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Scenes battle -SkipBuild
#>
param(
    [string[]]$Scenes = @("title_screen", "deck_builder", "battle"),
    [int]$Frames = 8,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -SkipBuild:$SkipBuild
