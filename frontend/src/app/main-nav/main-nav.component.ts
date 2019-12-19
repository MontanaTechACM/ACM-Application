import { Component, OnInit } from '@angular/core';
import { faBars } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-main-nav',
  templateUrl: './main-nav.component.html',
  styleUrls: ['./main-nav.component.sass']
})
export class MainNavComponent implements OnInit {

  faBars = faBars;
  openedSubMenu: boolean = false;

  constructor() { }

  ngOnInit() {
  }

  toggleSubMenu() {
    this.openedSubMenu = !this.openedSubMenu;
  }

}
