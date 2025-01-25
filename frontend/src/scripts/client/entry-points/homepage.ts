import { StoredCredential } from "../models/StoredCredential.ts";

const init = async () => {
  const $form = document.getElementById("app-homepage-form");

  if (!$form) {
    console.error("Webterm: Form not found");
    return;
  }

  $form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const $deviceName = $form.querySelector("input[name='device-name']") as HTMLInputElement;
    const $secretKey = $form.querySelector("input[name='secret-key']") as HTMLInputElement;
    const deviceName = $deviceName.value;
    const secretKey = $secretKey.value;

    const { index, storeKey } = await StoredCredential.store(deviceName, secretKey);

    window.location.href = `/terminal?store_index=${index}&store_key=${storeKey}`;
  });
};

await init();
