<script>
    import { FileUploader} from 'carbon-components-svelte';

    // Property to pass WASM function
    export let parse;

    let filesUpload = []; // file handles will be added in this
    let contentRaw = ""; // for us to see the contents
    let contentParsed = ""; // for us to see the contents

    let reader = new FileReader();
    // add listener to load event, which fires when file read is completed successfully
    reader.addEventListener("load",()=>{
        contentRaw = reader.result;
        // call WASM function to parse content
        contentParsed = parse(reader.result);
    },false);
    // this will be used as callback to file uploader
    let add_handler = (e) => {
        reader.readAsText(e.detail[0]);
    }

    // console.log(parse("A,b,c;5;6.3;7.8"));
</script>

<!-- scoped to component -->
<style>
/* unused CSS example - is not bundled -> warning during build */
    .icon-box {
        display: flex;
    }
</style>

    <FileUploader
            labelTitle="Upload file"
            buttonLabel="Add file"
            labelDescription="Only txt files are accepted."
            accept={[".txt"]}
            bind:filesUpload
            status="complete"
            on:add={add_handler}
    />
    Parsed Content: {JSON.stringify(contentParsed)}