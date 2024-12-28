import { StoredCredential } from "../models/StoredCredential.ts";

const init = async () => {
  const $form = document.getElementById("app-homepage-form");

  if (!$form) {
    console.error("Webterm: Form not found");
    return;
  }

  $form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const $serverId = $form.querySelector("input[name='server-id']") as HTMLInputElement;
    const $secretKey = $form.querySelector("input[name='secret-key']") as HTMLInputElement;
    const serverId = $serverId.value;
    const secretKey = $secretKey.value;

    const { index, storeKey } = await StoredCredential.store(serverId, secretKey);

    window.location.href = `/terminal?store_index=${index}&store_key=${storeKey}`;
  });
};

await init();
