# Native occupied-target probe for Craft #2183

This isolated validation child has sole parent `f367fe8fd0f054bbe2320742eac6fd853d6dbf91`
and adds exactly nine validation-only files. Production source and its lockfile are unchanged.
The workflow uses read-only permissions, pinned actions and one exact fork-only creation ref.

Four cases compare `atomicwrites` 0.4.4 `move_atomic` and `replace_atomic` with an occupied
target. The source is exclusively locked using the published directory-relative opening
helper. One pair keeps a `same_file::Handle` open on the target; the other pair leaves it
closed and creates a hard-link identity witness without keeping either name open. Target
identity handles are opened only after the operation in the closed case.

The closed-target overwrite must actually succeed and move the original locked source to
the destination. Both no-replace cases must fail with source/target bytes and identities
preserved. All cases verify the original lock remains exclusive. The held-target overwrite
outcome is observed, including its actual error kind and OS code, rather than assumed.
This diagnoses whether a target identity handle masks the negative control. It is not an
Engine test run and supplies no application-wide validation claim.

All fixtures are newly created in a uniquely named job-temporary directory on native NTFS.
No external or personal path is read or modified. The unchanged three child-opening helpers
come from the published baseline, with their original provenance retained below.
