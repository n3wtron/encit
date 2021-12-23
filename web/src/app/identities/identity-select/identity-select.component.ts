import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';
import {Identity, IdentityService} from "../identity.service";

@Component({
  selector: 'app-identity-select',
  templateUrl: './identity-select.component.html',
  styleUrls: ['./identity-select.component.scss']
})
export class IdentitySelectComponent implements OnInit {
  @Input()
  identity: Identity | undefined;

  @Output()
  identityChange = new EventEmitter<Identity | undefined>();

  setModel(newIdentity: Identity | undefined) {
    this.identity = newIdentity;
    this.identityChange.emit(newIdentity);
  }

  identities: Identity[] = [];

  constructor(private readonly identityService: IdentityService) {
  }

  ngOnInit(): void {
    this.identityService.getIdentities().subscribe(identities => this.identities = identities);
  }

}
