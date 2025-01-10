use solana_idlgen::idlgen;

idlgen!({
    "version": "0.1.0",
    "name": "turbin3_prereq",
    "instructions": [{
        "name": "complete",
        "accounts": [{
            "name": "signer",
            "isMut": true,
            "isSigner": true
        },
        {
            "name": "prereq",
            "isMut": true,
            "isSigner": false
        },
        {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
        }],
        "args": [{
            "name": "github",
            "type": "bytes"
        }]
    }],
    "metadata": {
        "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
    }
});