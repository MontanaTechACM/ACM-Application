import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-login-pop-up',
  templateUrl: './login-pop-up.component.html',
  styleUrls: ['./login-pop-up.component.sass']
})
export class LoginPopUpComponent implements OnInit {

  visible: boolean = false;

  constructor() {

  }

  ngOnInit() {

  }

  toggleLogin() {
    this.visible = !this.visible;
  }

}
