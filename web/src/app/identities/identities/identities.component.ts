import { Component, OnInit } from '@angular/core';
import { Identity, IdentityService } from '../identity.service';
import { MatDialog } from '@angular/material/dialog';
import { NewIdentityComponent } from '../new-identity/new-identity.component';
import { identity } from 'rxjs';
import * as FileSaver from 'file-saver';
import { Clipboard } from '@angular/cdk/clipboard';
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-identities',
  templateUrl: './identities.component.html',
  styleUrls: ['./identities.component.scss'],
})
export class IdentitiesComponent implements OnInit {
  identities: Identity[] = [];

  constructor(
    private readonly identityService: IdentityService,
    private readonly dialog: MatDialog,
    private readonly clipboard: Clipboard,
    private readonly snackBar: MatSnackBar
  ) {}

  ngOnInit(): void {
    this.refresh();
  }

  refresh() {
    this.identityService
      .getIdentities()
      .subscribe((identities) => (this.identities = identities));
  }

  open_new_identity_dialog() {
    let dialogRef = this.dialog.open(NewIdentityComponent, {
      panelClass: 'no-padded-dialog',
    });
    dialogRef.afterClosed().subscribe(() => {
      this.refresh();
    });
  }

  exportPublicKeyToFile(identity: Identity) {
    let blob = new Blob([identity.publicKey]);
    FileSaver.saveAs(blob, `${identity.name}.pub.pem`);
  }

  exportPublicKeyToClipboard(identity: Identity) {
    this.clipboard.copy(identity.publicKey);
    this.snackBar.open('Public key copied', undefined, { duration: 1000 });
  }
}
