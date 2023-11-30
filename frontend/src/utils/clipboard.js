

export async function copyToClipboard(input){
    let copied = await navigator.clipboard.writeText(input);
    return copied 
}
  