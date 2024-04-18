import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { u32 } from "@polkadot/types";
import { EventRecord } from "@polkadot/types/interfaces";
// import '@polkadot/api-augment';

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

const WEB_SOCKET = 'ws://127.0.0.1:9944';
const connect = async () => {
    const wsProvide = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({provider: wsProvide, types: {}});
    await api.isReady;
    return api;
}

const observe = async (api: ApiPromise) => {
    api.query.templateModule.something((result: u32) => {
        console.log(`Something is updated: ${result}`);
    })
}

const subscribe = async (api: ApiPromise) => {
    await api.query.system.events((events: EventRecord[]) => {
        events.forEach(({ phase, event: { data, method, section } }) => {
            if (section == "templateModule" && method == "SomethingStored") {
                console.log(`Event SomethingStored in template happened! :: ${data}`)
            }
          });
    })
}

const main = async () => {
    const api = await connect();

    await observe(api);

    await subscribe(api);

    await sleep(50000);

    console.log('main function');
}

main()
.then(() => {
    console.log('exits with success');
    process.exit(0);
})
.catch(err => {
    console.log('error is ', err);
    process.exit(1);
})