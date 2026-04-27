$ErrorActionPreference = 'Stop'

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$TargetDir = Join-Path $HOME '.agents/skills'

New-Item -ItemType Directory -Path $TargetDir -Force | Out-Null

$installedCount = 0

Get-ChildItem -Path $ScriptDir -Directory | ForEach-Object {
    $skillName = $_.Name

    if ($skillName.StartsWith('.')) {
        return
    }

    $skillFile = Join-Path $_.FullName 'SKILL.md'
    if (-not (Test-Path -Path $skillFile -PathType Leaf)) {
        return
    }

    $linkPath = Join-Path $TargetDir $skillName

    if (Test-Path -Path $linkPath) {
        Remove-Item -Path $linkPath -Recurse -Force
    }

    New-Item -ItemType SymbolicLink -Path $linkPath -Target $_.FullName | Out-Null
    Write-Host "Linked $skillName -> $($_.FullName)"
    $installedCount++
}

Write-Host "Installed $installedCount skill(s) to $TargetDir."
