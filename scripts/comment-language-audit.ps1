param(
    [string[]] $Paths = @("src"),
    [switch] $ShowExamples,
    [int] $ExampleLimit = 50
)

$asciiLetterPattern = '[A-Za-z]'
$cjkPattern = '\p{IsCJKUnifiedIdeographs}'
$excludePattern = '(^|[\\/])(target|\.git)([\\/]|$)'

$files = foreach ($path in $Paths) {
    if (Test-Path $path) {
        Get-ChildItem -Path $path -Recurse -File -Include *.rs,*.md,*.toml,*.yml,*.yaml,*.json
    }
}

$matches = foreach ($file in $files) {
    if ($file.FullName -match $excludePattern) {
        continue
    }

    $lineNumber = 0
    $inBlockComment = $false
    $extension = $file.Extension.ToLowerInvariant()

    foreach ($line in Get-Content -LiteralPath $file.FullName) {
        $lineNumber++
        $trimmed = $line.Trim()
        $isComment = $false

        switch ($extension) {
            ".rs" {
                if ($inBlockComment) {
                    $isComment = $true
                    if ($trimmed -match '\*/') {
                        $inBlockComment = $false
                    }
                } elseif ($trimmed -match '^(//|///|//!|/\*)') {
                    $isComment = $true
                    if ($trimmed -match '^/\*' -and $trimmed -notmatch '\*/') {
                        $inBlockComment = $true
                    }
                }
            }
            ".md" {
                $isComment = $trimmed -match '^<!--'
            }
            { $_ -in @(".toml", ".yml", ".yaml", ".ps1") } {
                $isComment = $trimmed -match '^#'
            }
        }

        if ($isComment -and $line -match $asciiLetterPattern -and $line -notmatch $cjkPattern) {
            [PSCustomObject]@{
                Path = Resolve-Path -Relative $file.FullName
                Line = $lineNumber
                Text = $trimmed
            }
        }
    }
}

$matches = @($matches)
$grouped = $matches | Group-Object Path | Sort-Object Count -Descending

[PSCustomObject]@{
    FilesScanned = @($files).Count
    CandidateLines = $matches.Count
    FilesWithCandidates = $grouped.Count
} | Format-List

$grouped | Select-Object Count, Name | Format-Table -AutoSize

if ($ShowExamples) {
    $matches | Select-Object -First $ExampleLimit | Format-Table -AutoSize
}
