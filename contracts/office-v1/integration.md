# BreakFree integration contract v1

Status: implemented by the Office prototype; adoption by real editors is pending.
The JSON Schema in this directory is normative together with the transport and
semantic rules below. Examples are fictional test data, not installation records
or claims about available editor features.

## Repository and application boundaries

Office, Docs, Sheets and Slides keep separate source trees, builds, tests,
versions and executables. Each editor must create, open, save and reopen its
supported documents without Office or the other editors. Office only dispatches
requests; it never edits content. No submodules, sibling-directory build imports,
copies of editor code or dependencies on mutable `main` branches are required.
Future shared libraries require a demonstrated need and an immutable version.

| Application | Stable ID | Provisional native format ID | Extension |
| --- | --- | --- | --- |
| BreakFree Docs | `breakfree.docs` | `breakfree.docs.native` | `bfdoc` |
| BreakFree Sheets | `breakfree.sheets` | `breakfree.sheets.native` | `bfsheet` |
| BreakFree Slides | `breakfree.slides` | `breakfree.slides.native` | `bfslide` |

These extensions are internal prototype conventions, not OS associations. A
document format's version is independent of this integration protocol. The
contract does not prescribe a document serialization or imply Office-format
support. Advertise DOCX, XLSX or PPTX only after implementation and round-trip
tests in the editor establish precisely what is preserved.

## Executable interface

The user explicitly selects a trusted executable for each stable application ID.
Manifests and document contents cannot specify executable paths or arguments.
Office does not search PATH, inspect documents for commands, download apps, use
a shell, register file associations, or open files using an OS default handler.

| Argument vector after the executable | Required behavior |
| --- | --- |
| `["--capabilities"]` | Write exactly one UTF-8 JSON object to stdout and exit 0, without GUI. |
| `["--version"]` | Write the actual build version as one UTF-8 line and exit 0, without GUI. |
| `["--new"]` | Open a new document of the application's own type if `new` is advertised. |
| `["--open", absolute_path]` | Open the named existing local file if `open` and its readable format are advertised. |

An executable must recognize these options before normal GUI startup. It must
reject unknown or malformed options with a nonzero exit code. Paths are single
arguments even when they contain spaces, Unicode, quotes or shell punctuation.
Editors must not reinterpret them as commands. Office resolves selected paths to
absolute paths, matches the last extension case-insensitively and passes only
the selected path. It does not parse document content or promise that a file is
valid merely because its extension matches.

## Capability response

Required fields: `protocol_version` (the JSON integer `1`, not a boolean, string
or fractional representation), `app_id`, actual `app_version`, `display_name`,
`capabilities` and `formats`. Version strings are nonempty build identifiers;
v1 does not require SemVer or compare application versions for compatibility.
Protocol compatibility is exact: Office v1 accepts protocol 1 only. All six
fields must conform to `app-manifest.schema.json`; unknown fields are rejected.
Whitespace around the JSON object is allowed. Duplicate JSON keys, non-finite
numbers, invalid UTF-8, trailing data and multiple objects are rejected.
Names and versions must contain valid Unicode scalar text, with no unpaired
surrogates introduced by JSON escapes; display names cannot be whitespace-only.

Additional semantic checks:

- The returned `app_id` must equal the card to which the user assigned the executable.
- Format IDs must be unique within an application.
- Extensions must be unambiguous within an application, lowercase and without dots.
- A readable format requires the `open` action. A `new` action does not require
  a format: an editor may support only unsaved new documents.
- Each format must support reading or writing. `write` describes the editor's
  save ability; v1 does not expose a save or conversion command to Office.

Office chooses an app for Open only among trusted, successfully probed apps
advertising `open` and `read: true` for that extension. When multiple apps match,
the user chooses; Office never silently guesses. Create uses the selected card
and requires `new`. There are no hard-coded native-format support claims in the
launcher. An editor may initially advertise fewer actions and formats.

## Limits and errors

The capability probe has a 3-second deadline covering output collection and
process exit, a 65,536-byte stdout limit and a separate 16,384-byte stderr limit.
It requires a successful exit, then validates JSON and identity. Diagnostics go
to stderr, never stdout. Overflow or timeout stops the probe; a child holding
an inherited output pipe open cannot hold the UI indefinitely. Office must
keep probes and process waiting off the GUI thread. Size limits apply before
parsing; deeply nested JSON must fail safely as well.

Office reports missing/unusable executable, launch failure, timeout, excessive
output, nonzero probe exit, invalid JSON/manifest, wrong identity, unsupported
protocol and unsupported action/format. Missing editors are a normal state:
the window remains usable and offers configuration. Output is untrusted data
and cannot supply commands, executable paths or shell fragments.

The prototype re-probes before every dispatch, so capability and protocol
changes are detected. A successful dispatch means the process started and did
not immediately report failure; it does not prove that the editor opened or
saved the file. v1 has no delivery acknowledgement or content exchange.

Executable selection grants local-code execution, including the initial probe.
This is a trust boundary, not a sandbox or signature verification. Replacing a
trusted binary may change the code that runs. Settings are local to the user;
do not import settings from a document or an untrusted source. On Windows,
Office accepts `.exe` files only, avoiding batch-file shell interpretation. On
macOS, choose the actual binary inside `Application.app/Contents/MacOS/`, not
the bundle directory. POSIX probe cleanup kills the probe's process group;
Windows cleanup kills the direct probe process and closes its pipes. Descendant
termination on Windows is not guaranteed in this prototype.

## Editor adoption checklist

1. Implement the four executable commands without depending on Office.
2. Validate live `--capabilities` output against the schema and semantic rules.
3. Use the stable app ID and an actual build version; copy no example claims.
4. Test native create/save/reopen independently, then test Office dispatch with
   the real build, paths with spaces and Unicode, and a recorded OS/version.
5. Record the Office, editor and protocol versions, results and known losses.

Breaking protocol changes require a new versioned directory and an explicit
compatibility policy. Never change v1 to silently accept an incompatible v2.
