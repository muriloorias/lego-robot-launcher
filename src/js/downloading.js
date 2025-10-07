import { invoke } from "@tauri-apps/api/tauri";



async function VerifyIsDownloaded() {
    await invoke("verify_downloaded");

    const isInstalled = await invoke("get_downloaded");

    let isIntalledText = document.getElementById('isDownloadedText');
    let isInstalledIcon = document.getElementById('isDowloadedIcon');
    let localizeinstalation = document.getElementById('localizeDownload');

    if (isInstalled) {
        isIntalledText.textContent = "O jogo está instalado";
        isInstalledIcon.style.display = 'none';
        localizeinstalation.style.display = 'none';
    } else {
        isIntalledText.textContent = "O jogo ainda não foi instalado";
        isInstalledIcon.style.display = 'block';
        localizeinstalation.style.display = 'block';
    }
}
VerifyIsDownloaded();
