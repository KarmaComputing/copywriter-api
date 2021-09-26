# Store edited landing pages

As a copywriter, the edits I make to a site
are stored somehwere, so then I can preview the changes
made to the document.

As an engineer, every save a copywriter performs, causes
a HTTP POST request to store the entire DOM of the page
they have edited, with a timestamp, so that the edits
may be stored, and viewed in the future.


Future: Use [mutationobserver](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver) to record an playback changes.

Ref https://github.com/KarmaComputing/copyeditor
