var openAsModal = function openAsModal(target) {
    var parent = document.getElementById("parent");
    parent.style.display = "none";
    var modal = document.getElementById("modal");
    modal.innerHTML = target.innerHTML;
    modal.style.display = "block";
};

var closeModal = function () {
    var modal = document.getElementById("modal");
    modal.style.display = "none";
    modal.innerHTML = "";

    var parent = document.getElementById("parent");
    parent.style.display = "flex";
}

document.addEventListener("DOMContentLoaded", function () {
    var panes = document.getElementsByClassName("pane");

    for (var i = 0; i < panes.length; i++) {
        var pane = panes[i];
        pane.onclick = function (event) {
            openAsModal(event.target);
        };
    }
});
