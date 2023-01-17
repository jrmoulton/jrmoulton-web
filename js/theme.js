/* When the user clicks on the button,
        toggle between hiding and showing the dropdown content */
function myFunction() {
  {
    document.getElementById("myDropdown").classList.toggle("show");
  }
}

// Close the dropdown menu if the user clicks outside of it
window.onclick = function (event) {
  {
    if (!event.target.matches(".dropbtn")) {
      {
        const dropdowns = document.getElementsByClassName("dropdown-content");
        if (dropdowns[0].classList.contains("show")) {
          {
            dropdowns[0].classList.remove("show");
          }
        }
      }
    }
  }
};

function setTheme(button) {
  {
    const theme = button.getAttribute("href");
    document.getElementById("theme").href = theme;

    // Store a value in local storage
    window.localStorage.setItem("theme", theme);
  }
}

window.onload = function () {
  let value = localStorage.getItem("theme");
  if (!value) {
    value = "/styles/onedark_dark.css";
    localStorage.setItem("theme", value);
  }
  document.getElementById("theme").href = value;
};
