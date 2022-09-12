// resolution of browser for kindle paperwhite is 1072x1448
var originalDisplay = "none";

var openAsModal = function openAsModal(target) {
    // hide parent
    var parent = document.getElementById("parent");
    originalDisplay = parent.style.display;
    parent.style.display = "none";

    // set modal content modal
    var modalContent = document.getElementById("modalContent");
    modalContent.innerHTML = target.innerHTML;

    // show modal
    var modal = document.getElementById("modal");
    modal.style.display = "block";
};

var closeModal = function () {
    var modal = document.getElementById("modal");
    modal.style.display = "none";

    var parent = document.getElementById("parent");
    parent.style.display = originalDisplay;
}

document.addEventListener("DOMContentLoaded", function () {
    var panes = document.getElementsByClassName("pane");

    for (var i = 0; i < panes.length; i++) {
        var pane = panes[i];
        pane.onclick = function (event) {
            event.preventDefault();
            event.stopPropagation();
            openAsModal(event.currentTarget);
        };
    }
});