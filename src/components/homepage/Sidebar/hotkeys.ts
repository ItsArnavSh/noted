import hotkeys from "hotkeys-js";

hotkeys("f5", function (event, handler) {
  event.preventDefault();
  alert("you pressed F5!");
});
