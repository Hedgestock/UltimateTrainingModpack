var isNx = (typeof window.nx !== 'undefined');
var defaults_prefix = "__";

if (isNx) {
    window.nx.footer.setAssign('B', '', close_or_exit, {se: ''});
    window.nx.footer.setAssign('X', '', resetCurrentSubmenu, {se: ''});
    window.nx.footer.setAssign('L', '', resetAllSubmenus, {se: ''});
    window.nx.footer.setAssign('R', '', saveDefaults, {se: ''});
    window.nx.footer.setAssign('ZR', '', cycleNextTab, {se: ''});
    window.nx.footer.setAssign('ZL', '', cyclePrevTab, {se: ''});
} else {
    document.addEventListener('keypress', (event) => {
        switch (event.key) {
            case "b":
                console.log("b");
                close_or_exit();
                break;
            case "x":
                console.log("x");
                resetCurrentSubmenu();
                break;
            case "l":
                console.log("l");
                resetAllSubmenus();
                break;
            case "r":
                console.log("r");
                saveDefaults();
                break;
            case "p":
                console.log("p");
                cycleNextTab();
                break;
            case "o":
                console.log("o");
                cyclePrevTab();
                break;
        }
    });
}

window.onload = onLoad;
var settings = new Map();
var lastFocusedItem = document.querySelector(".menu-item > button");

function onLoad() {
    // Activate the first tab
    openTab(document.querySelector("button.tab-button"));

    // Extract URL params and set appropriate settings
    setSettingsFromURL();
    setSubmenusFromSettings();
}

function openTab(e) {
    var tab_id = e.id.replace("button", "tab");
    var selected_tab = document.getElementById(tab_id);


    // Hide all content for all tabs
    closeAllItems();
    tabcontent = document.getElementsByClassName("tab-content");
    Array.from(tabcontent).forEach(element => {
        element.classList.add("hide");
    });


    // Get all elements with class="tablinks" and remove the class "active"
    tablinks = document.getElementsByClassName("tab-button");
    Array.from(tablinks).forEach(element => {
        element.classList.remove("active");
    });

    // Show the current tab, and add an "active" class to the button that opened the tab
    e.classList.add("active");
    selected_tab.classList.remove("hide");
    selected_tab.querySelector("button").focus();
}

function openItem(e) {
    playSound("SeWebMenuListOpen");
    var modal = e.parentElement.querySelector(".modal");
    modal.classList.toggle("hide");
    modal.querySelector("button").focus();
    lastFocusedItem = e;
}

function closeAllItems() {
    var modals = document.querySelectorAll(".modal");
    Array.from(modals).forEach(element => {
        element.classList.add("hide");
    });
    lastFocusedItem.focus();
}

function toggleOption(e) {
    playSound("SeSelectCheck");
    if (e.parentElement.classList.contains("single-option")) {
        selectSingleOption(e);
    } else {
        var img = e.querySelector("img");
        img.classList.toggle("hide");
    }
}

function closestClass(elem, class_) {
    // Returns the closest ancestor (including self) with the given class
    // TODO: Consider removing
    if (!elem) {
        // Reached the end of the DOM
        return null
    } else if (elem.classList.contains(class_)) {
        // Found it
        return elem
    } else {
        // Didn't find it, go up a level
        return closestClass(elem.parentElement, class_);
    }
}
function playSound(label) {
    // Valid labels:
    // SeToggleBtnFocus
    // SeToggleBtnOn
    // SeToggleBtnOff
    // SeCheckboxFocus
    // SeCheckboxOn
    // SeCheckboxOff
    // SeRadioBtnFocus
    // SeRadioBtnOn
    // SeSelectCheck
    // SeSelectUncheck
    // SeBtnDecide
    // SeTouchUnfocus
    // SeBtnFocus
    // SeKeyError
    // SeDialogOpen
    // SeWebZoomOut
    // SeWebZoomIn
    // SeWebNaviFocus
    // SeWebPointerFocus
    // SeFooterFocus
    // SeFooterDecideBack
    // SeFooterDecideFinish
    // SeWebChangeCursorPointer
    // SeWebTouchFocus
    // SeWebLinkDecide
    // SeWebTextboxStartEdit
    // SeWebButtonDecide
    // SeWebRadioBtnOn
    // SeWebCheckboxUncheck
    // SeWebCheckboxCheck
    // SeWebMenuListOpen
    if (isNx) {
        window.nx.playSystemSe(label);
    } else {
        console.log("Sound Effect: " + label);
    }
}

function exit() {
    playSound("SeFooterDecideBack");
    setSettingsFromMenu();
    var url = buildURLFromSettings();

    if (isNx) {
        window.location.href = url;
    } else {
        console.log(url);
    }
}

function close_or_exit() {
    // If any submenus are open, close them
    // Otherwise if all submenus are closed, exit the menu and return to the game

    if (document.querySelector(".modal:not(.hide)")) {
        // Close any open submenus
        console.log("Closing Items");
        closeAllItems();
    } else {
        // If all submenus are closed, exit and return through localhost
        console.log("Exiting");
        exit();
    }
}

function setSettingsFromURL() {
    var regex = /[?&]([^=#]+)=([^&#]*)/g,
        match;
    while (match = regex.exec(document.URL)) {
        settings.set(match[1], match[2]);
    }
}

function setSettingsFromMenu() {
    var section;
    var mask;
    [].forEach.call(document.querySelectorAll(".menu-item"), function (menuItem) {
        section = menuItem.id;
        mask = getMaskFromSubmenu(menuItem);
        settings.set(section, mask);
    });
}

function buildURLFromSettings() {
    var url = "http://localhost/";
    url += "?";
    settings.forEach((val, key) => { url += key + "=" + String(val) + "&" } );
    return url
}

function selectSingleOption(e) {
    // Deselect all options in the submenu
    parent = closestClass(e, "single-option");
    siblings = parent.querySelectorAll(".menu-icon img");
    [].forEach.call(siblings, function (sibling) {
        sibling.classList.add("hide");
    });
    e.querySelector(".menu-icon img").classList.remove("hide");
}

function setSubmenusFromSettings() {
    [].forEach.call(document.querySelectorAll(".menu-item"), function (menuItem) {
        var section = menuItem.id;
        var section_mask = decodeURIComponent(settings.get(section));
        setSubmenuByMask(menuItem, section_mask)
    });
}

function setSubmenuByMask(menuItem, mask) {
    [].forEach.call(menuItem.querySelectorAll(".modal .menu-icon img"), function (toggle) {
        if (isInBitmask(toggle.dataset.val, mask)) {
            toggle.classList.remove("hide");
        } else {
            toggle.classList.add("hide");
        }
    });

    // If no setting for a Single Option is set, select the first one
    var isSingleOption = menuItem.querySelectorAll(".modal.single-option").length != 0;
    var isAllDeselected = menuItem.querySelectorAll(".modal .menu-icon img:not(.hide)").length == 0;
    if (isSingleOption & isAllDeselected) {
        selectSingleOption(menuItem.querySelector(".modal button"));
    }
}

function getMaskFromSubmenu(menuItem) {
    var val = 0;
    [].forEach.call(menuItem.querySelectorAll(".modal img:not(.hide)"), function (toggle) {
            val += parseInt(toggle.dataset.val);
    });
    return val
}

function resetCurrentSubmenu() {
    var focus = document.querySelector(".menu-item .modal:not(.hide)");
    if (!focus) {
        focus = document.querySelector(":focus");
    }
    var menuItem = closestClass(focus, "menu-item");

    var key = defaults_prefix + menuItem.id;
    var section_mask = decodeURIComponent(settings.get(key));
    setSubmenuByMask(menuItem, section_mask);
}

function resetAllSubmenus() {
    // Resets all submenus to the default values
    if (confirm("Are you sure that you want to reset all menu settings to the default?")) {
        [].forEach.call(document.querySelectorAll(".menu-item"), function (menuItem) {
            var key = defaults_prefix + menuItem.id;
            var mask = decodeURIComponent(settings.get(key));
            setSubmenuByMask(menuItem, mask)
        });
    }
}

function setHelpText(text) {
    // Modify the help text in the footer
    document.getElementById("help-text").innerText = text;
}

function saveDefaults() {
    if (confirm("Are you sure that you want to change the default menu settings to the current selections?")) {
        var key;
        var mask;
        [].forEach.call(document.querySelectorAll(".menu-item"), function (menuItem) {
            key = defaults_prefix + menuItem.id;
            mask = getMaskFromSubmenu(menuItem);
            settings.set(key, mask);
        });
    }
}

function isInBitmask(val, mask) {
    // Return true if the value is in the bitmask
    return (mask & val) != 0
}

function cycleNextTab() {
    // Cycle to the next tab
    var activeTab = document.querySelector(".tab-button.active");
    var nextTab = activeTab.nextElementSibling;
    if (!nextTab) {
        // On the last tab - set the next tab as the first tab in the list
        nextTab = document.querySelector(".tab-button");
    }
    openTab(nextTab);
}

function cyclePrevTab() {
    // Cycle to the previous tab
    var activeTab = document.querySelector(".tab-button.active");
    var prevTab = activeTab.previousElementSibling;
    if (!prevTab) {
        // On the first tab - set the next tab as the last tab in the list
        tabs = document.querySelectorAll(".tab-button");
        prevTab = tabs[tabs.length - 1];
    }
    openTab(prevTab);
}