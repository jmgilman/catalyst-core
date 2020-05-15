const wasm = require('wallet-cordova-plugin.wasmModule');

async function walletRestore (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'string') {
        const mnemonics = opts[0];
        const password = '';
        try {
            const wallet = wasm.Wallet.recover(mnemonics, password);
            successCallback(wallet.ptr);
        } catch (err) {
            errorCallback(`couldn't recover wallet ${err}`);
        }
    } else {
        errorCallback('no mnemonics provided');
    }
}

async function walletRetrieveFunds (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number' && opts[1] instanceof ArrayBuffer) {
        const walletPtr = opts[0];
        const block = opts[1];

        const wallet = wasm.Wallet.__wrap(walletPtr);

        try {
            const settings = wallet.retrieve_funds(new Uint8Array(block));
            successCallback(settings.ptr);
        } catch (err) {
            errorCallback(`couldn't retrieve funds ${err}`);
        }
    } else {
        errorCallback('missing walletPtr or block');
    }
}

async function walletTotalFunds (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number') {
        const walletPtr = opts[0];
        const wallet = wasm.Wallet.__wrap(walletPtr);

        try {
            successCallback(wallet.total_value());
        } catch (err) {
            errorCallback(`couldn't get funds ${err}`);
        }
    } else {
        errorCallback('no pointer');
    }
}

async function walletId (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number') {
        const walletPtr = opts[0];
        const wallet = wasm.Wallet.__wrap(walletPtr);

        try {
            successCallback(wallet.id());
        } catch (err) {
            errorCallback(`couldn't get funds ${err}`);
        }
    } else {
        errorCallback('no pointer');
    }
}

async function walletConvert (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number' && typeof (opts[1]) === 'number') {
        const walletPtr = opts[0];
        const settingsPtr = opts[1];
        const wallet = wasm.Wallet.__wrap(walletPtr);
        const settings = wasm.Settings.__wrap(settingsPtr);

        try {
            successCallback(wallet.convert(settings).ptr);
        } catch (err) {
            errorCallback(`couldn't get funds ${err}`);
        }
    } else {
        errorCallback('no pointer');
    }
}

async function conversionTransactionsSize (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number') {
        const conversionPtr = opts[0];
        const conversion = wasm.Conversion.__wrap(conversionPtr);

        try {
            successCallback(conversion.transactions_len());
        } catch (err) {
            errorCallback(`couldn't get transactions size: ${err}`);
        }
    } else {
        errorCallback('no pointer');
    }
}

async function conversionTransactionsGet (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number' && typeof (opts[1]) === 'number') {
        const conversionPtr = opts[0];
        const index = opts[1];
        const conversion = wasm.Conversion.__wrap(conversionPtr);

        try {
            successCallback(conversion.transactions_get(index));
        } catch (err) {
            errorCallback(`couldn't get transaction at index: ${index} - error: ${err}`);
        }
    } else {
        errorCallback('no pointer');
    }
}

async function walletDelete (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number') {
        const walletPtr = opts[0];
        wasm.Wallet.__wrap(walletPtr).free();
        successCallback();
    } else {
        errorCallback();
    }
}

async function settingsDelete (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number') {
        const settingsPtr = opts[0];
        wasm.Settings.__wrap(settingsPtr).free();
        successCallback();
    } else {
        errorCallback();
    }
}

async function conversionDelete (successCallback, errorCallback, opts) {
    await loaded;
    if (opts && typeof (opts[0]) === 'number') {
        const conversionPtr = opts[0];
        wasm.Conversion.__wrap(conversionPtr).free();
        successCallback();
    } else {
        errorCallback();
    }
}

const bindings = {
    WALLET_RESTORE: walletRestore,
    WALLET_RETRIEVE_FUNDS: walletRetrieveFunds,
    WALLET_TOTAL_FUNDS: walletTotalFunds,
    WALLET_ID: walletId,
    WALLET_CONVERT: walletConvert,
    CONVERSION_TRANSACTIONS_SIZE: conversionTransactionsSize,
    CONVERSION_TRANSACTIONS_GET: conversionTransactionsGet,
    WALLET_DELETE: walletDelete,
    SETTINGS_DELETE: settingsDelete,
    CONVERSION_DELETE: conversionDelete
};

require('cordova/exec/proxy').add('WalletPlugin', bindings);

// this is in done in order to make it work regardless of where the html file is located
const appPath = global.require('electron').remote.app.getAppPath();
const binaryPath = global.require('path').join(appPath, 'wallet_js_bg.wasm');
// TODO: we could probably do this async
const bytes = global.require('fs').readFileSync(binaryPath);

const loaded = wasm(bytes);
