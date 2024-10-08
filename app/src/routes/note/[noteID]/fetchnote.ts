export function fetchNote(noteID: string) {
  return fetch(`/embed/${noteID}`)
    .then((res) => {
      return res.text();
    })
}
